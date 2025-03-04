pub mod app;
pub mod daemon;
pub mod hw;
pub mod installer;
pub mod launcher;
pub mod loader;
pub mod logger;
pub mod signer;
pub mod ui;
pub mod utils;

use liana::Version;

pub const VERSION: Version = Version {
    major: 0,
    minor: 3,
    patch: 0,
};
