use directories::UserDirs;
use rfd::AsyncFileDialog;
use std::path::PathBuf;

#[allow(unused)]
/// Platform-specific functionality like opening dialogs, interacting with recycle bins, etc.
pub struct Platform;

#[allow(unused)]
impl Platform {
    pub async fn open_file_picker() -> Option<PathBuf> {
        let user_dirs = UserDirs::new()?;

        let default_dir = user_dirs
            .download_dir()
            .or_else(|| user_dirs.document_dir())
            .or_else(|| Some(user_dirs.home_dir()))?;

        Some(
            AsyncFileDialog::new()
                .add_filter("Comic Books", &["cbz"])
                .set_directory(default_dir)
                .pick_file()
                .await?
                .path()
                .into(),
        )
    }
}
