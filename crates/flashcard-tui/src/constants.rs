use directories::ProjectDirs;
use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub const QUALIFIER: &'static str = "xyz";
pub const ORGANIZATION: &'static str = "lakrestofer";

pub fn project_name() -> &'static str {
    // static PROJECT_NAME: OnceLock<String> = OnceLock::new();
    env!("CARGO_CRATE_NAME")
}

pub fn project_dirs() -> &'static ProjectDirs {
    static PROJECT_DIRS: OnceLock<ProjectDirs> = OnceLock::new();
    PROJECT_DIRS.get_or_init(|| {
        ProjectDirs::from(QUALIFIER, ORGANIZATION, project_name())
            .expect("Could not fild valid home directory")
    })
}

/// path to the data directory of this application
pub fn data_dir_path() -> &'static Path {
    project_dirs().data_dir()
}

pub fn log_env() -> &'static str {
    static LOG_ENV: OnceLock<String> = OnceLock::new();
    LOG_ENV.get_or_init(|| format!("{}_LOGLEVEL", project_name()))
}

pub fn log_dir_path() -> &'static Path {
    static LOG_DIR: OnceLock<PathBuf> = OnceLock::new();
    LOG_DIR.get_or_init(|| {
        let mut path = PathBuf::from(data_dir_path());
        path.push("logs");
        path
    })
}

pub fn log_file_path() -> &'static Path {
    static LOG_FILE: OnceLock<PathBuf> = OnceLock::new();
    LOG_FILE.get_or_init(|| {
        log_dir_path()
            .to_path_buf()
            .join(format!("{}.log", env!("CARGO_PKG_NAME")))
    })
}

// pub fn get_data_dir() -> PathBuf {
//     let directory = if let Some(s) = DATA_FOLDER.clone() {
//         s
//     } else if let Some(proj_dirs) = project_directory() {
//         proj_dirs.data_local_dir().to_path_buf()
//     } else {
//         PathBuf::from(".").join(".data")
//     };
//     directory
// }
