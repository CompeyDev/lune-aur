use std::fs::File;

use action::download::{download_release, install_lune};
use tracing::Level;
use tracing_unwrap::ResultExt;

fn main() {
    if cfg!(debug_assertions) {
        better_panic::install();
    } else {
        // Check for is_debug in github actions
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }

    let (zip_path, meta) =
        download_release().expect_or_log("failed to download latest lune release");

    install_lune(
        File::open(&zip_path).expect(
            format!(
                "failed to open downloaded lune release zip file @ {}",
                zip_path.to_string_lossy().to_string()
            )
            .as_str(),
        ),
        meta,
    )
    .expect_or_log(
        "failed to install lune. did we not have perms to write to the required directories?",
    );
}
