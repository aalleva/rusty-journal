use std::path::PathBuf;

pub enum Action {
    Add { task: String },
    Delete { position: usize },
    List
}

pub struct CommandLineArgs {
    pub action: Action,
    pub journal_file: Option<PathBuf>
}
