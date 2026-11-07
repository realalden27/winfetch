#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::*;

#[cfg(not(windows))]
pub mod mock;

#[cfg(not(windows))]
pub use mock::*;
