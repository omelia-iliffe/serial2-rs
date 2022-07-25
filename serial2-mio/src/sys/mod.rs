#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use unix::Inner;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::Inner;
