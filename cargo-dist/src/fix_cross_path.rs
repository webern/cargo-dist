use camino::Utf8PathBuf;
use tracing::log::{trace, warn};
use tracing::{debug, info};

/// HACK - when using cargo cross, the path will be wrong. Instead we
/// will make two bold assumptions:
/// 1) When compiled with cross, the target directory will be mounted
///    at the root of the container, i.e. `/target/...`.
/// 2) The location of the target directory on the host is at the
///    the root of the cargo project.
/// This might not work on Windows.
pub(crate) fn fix_cross_path(path: Utf8PathBuf) -> Utf8PathBuf {
    if path.is_absolute() && path.starts_with("/target") {
        debug!(
            "The path '{}' is likely incorrect due to the use of cross-rs.",
            path
        );
    } else {
        trace!(
            "The path '{}' is probably fine, we will not change it.",
            path
        );
        return path;
    }
    let relative_path = match path.strip_prefix(&Utf8PathBuf::from("/")) {
        Ok(p) => p,
        Err(e) => {
            warn!("Unable to fix this suspected bad path '{}': {}", path, e);
            return path;
        }
    };

    trace!(
        "Stripped the absolute path / to create the path '{}'",
        relative_path
    );

    // Just to be super clear, let's add ./ to the beginning of this.
    let fixed_path = Utf8PathBuf::from(".").join(relative_path);
    info!(
        "Changed a suspected bad cross-rs path from '{}' to '{}'",
        path, fixed_path
    );
    fixed_path
}

#[test]
fn test_no_change_already_relative() {
    let path = Utf8PathBuf::from("/target/foo/bar");
    let expected = Utf8PathBuf::from("./target/foo/bar");
    let output = fix_cross_path(path);
    assert_eq!(expected, output);
}

#[test]
fn test_no_does_not_start_with_target() {
    let path = Utf8PathBuf::from("/home/foo/bar");
    let output = fix_cross_path(path.clone());
    assert_eq!(path, output);
}

#[test]
fn test_changed_to_relative() {
    let path = Utf8PathBuf::from("target/foo/bar");
    let output = fix_cross_path(path.clone());
    assert_eq!(path, output);
}
