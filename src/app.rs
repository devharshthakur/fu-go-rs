use crate::util::go_deleter::delete_go_installations;
use crate::util::go_deleter::DeleterError;
use crate::util::go_finder::find_go_installations;
use crate::util::go_finder::FinderError;
use crate::util::go_finder::GoInstallation;
use std::path::PathBuf;
use tokio::sync::mpsc;

pub enum AppMessage {
    GoFound(Result<GoInstallation, FinderError>),
    GoDeleted(Result<(), DeleterError>),
}

#[derive(PartialEq)]
pub enum AppState {
    Loading,
    Confirm,
    Deleting,
    Complete,
}

pub struct App {
    pub state: AppState,
    pub should_quit: bool,
    pub go_versions: Vec<String>,
    pub go_install_path: PathBuf,
    pub input: String, // For the "yes" confirmation
    pub spinner_frame: usize,
    pub error_message: Option<String>,
    pub deletion_complete: bool,
}

impl App {
    /// Creates a new App instance and starts an asynchronous task to find Go installations.
    /// The task will send a GoFound message when complete.
    pub fn new(sender: mpsc::Sender<AppMessage>) -> Self {
        // Kickoff a background(async) task to find go installations
        tokio::spawn(async move {
            let result = find_go_installations();
            sender.send(AppMessage::GoFound(result)).await.unwrap();
        });
        Self {
            state: AppState::Loading,
            should_quit: false,
            go_versions: Vec::new(),
            go_install_path: PathBuf::new(),
            input: String::new(),
            spinner_frame: 0,
            error_message: None,
            deletion_complete: false,
        }
    }

    /// Updates the spinner animation frame for the loading state.
    /// Called on each tick to animate the loading spinner.
    pub fn on_tick(&mut self) {
        self.spinner_frame = (self.spinner_frame + 1) % 8;
    }

    /// Handles incoming messages from background tasks and updates the app state accordingly.
    /// Processes GoFound and GoDeleted messages, updating the UI state and error messages.
    pub fn handle_message(&mut self, msg: AppMessage) {
        match msg {
            AppMessage::GoFound(Ok(installation)) => {
                // Handle GoFound
                self.go_versions = installation.versions;
                self.go_install_path = installation.path_to_delete;
                self.state = AppState::Confirm;
            }
            AppMessage::GoFound(Err(_)) => {
                // Handle find error
                self.error_message = Some("Failed to find Go installations".to_string());
                self.state = AppState::Complete;
            }
            AppMessage::GoDeleted(Ok(_)) => {
                // Handle successful deletion
                self.deletion_complete = true;
                self.state = AppState::Complete;
            }
            AppMessage::GoDeleted(Err(_)) => {
                // Handle deletion error
                self.error_message = Some("Failed to delete Go installations".to_string());
                self.state = AppState::Complete;
            }
        }
    }

    /// Initiates the deletion process if the user input is "yes" (case-insensitive).
    /// Starts an asynchronous deletion task and transitions to the Deleting state.
    /// If the input is not "yes", sets should_quit to true to exit the application.
    pub fn start_deletion(&mut self, sender: mpsc::Sender<AppMessage>) {
        if self.input.to_lowercase() == "yes" {
            self.state = AppState::Deleting;
            let path_to_delete = self.go_install_path.clone();

            tokio::spawn(async move {
                let result = delete_go_installations(&path_to_delete);
                sender.send(AppMessage::GoDeleted(result)).await.unwrap();
            });
        } else {
            self.should_quit = true;
        }
    }
}
