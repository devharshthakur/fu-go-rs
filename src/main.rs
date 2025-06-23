use crate::tui::ui;
use app::App;
use app::AppMessage;
use app::AppState;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use std::io;
use std::time::Duration;
use tokio::sync::mpsc;

pub mod app;
pub mod tui;
pub mod util;

#[tokio::main]
async fn main() -> io::Result<()> {
    let (tx, mut rx) = mpsc::channel(10);

    let mut terminal = tui::init()?;
    let mut app = App::new(tx.clone());

    while !app.should_quit {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if let Ok(message) = rx.try_recv() {
            app.handle_message(message);
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                handle_key_event(key, &mut app, tx.clone());
            }
        }

        app.on_tick();
    }

    tui::restore()?;
    Ok(())
}

fn handle_key_event(key: KeyEvent, app: &mut App, sender: mpsc::Sender<AppMessage>) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    match app.state {
        AppState::Complete => match key.code {
            KeyCode::Char('q') | KeyCode::Enter | KeyCode::Esc => {
                app.should_quit = true;
            }
            _ => {}
        },
        _ => match key.code {
            KeyCode::Char('c') if key.modifiers == event::KeyModifiers::CONTROL => {
                app.should_quit = true;
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                app.should_quit = true;
            }
            KeyCode::Enter if app.state == AppState::Confirm => {
                app.start_deletion(sender);
            }
            KeyCode::Char(c) if app.state == AppState::Confirm => {
                app.input.push(c);
            }
            KeyCode::Backspace if app.state == AppState::Confirm => {
                app.input.pop();
            }
            _ => {}
        },
    }
}
