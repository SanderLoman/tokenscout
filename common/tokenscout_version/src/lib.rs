// Copied from https://github.com/sigp/lighthouse/blob/f1d88ba4b1256e084d3a2a547e0ce574b896246c/common/lighthouse_version/src/lib.rs

use git_version::git_version;
use target_info::Target;

pub const VERSION: &str = git_version!(
    args = [
        "--always",
        "--dirty=+",
        "--abbrev=7",
        "--match=thiswillnevermatchlol"
    ],
    prefix = "tokenscout/v0.1.0-",
    fallback = "tokenscout/v0.1.0"
);

pub const COMMIT_PREFIX: &str = git_version!(
    args = ["--always", "--abbrev=8", "--match=thiswillnevermatchlol"],
    prefix = "",
    suffix = "",
    cargo_prefix = "",
    cargo_suffix = "",
    fallback = "00000000"
);

pub fn version_with_platform() -> String {
    format!("{}/{}-{}", VERSION, Target::arch(), Target::os())
}
