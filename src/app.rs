use crate::go_deleter::{delete_go_installations, DeleterError};
use crate::go_finder::{find_go_installations, FinderError, GoInstallation};
use std::path::PathBuf;
use tokio::sync::mpsc;
