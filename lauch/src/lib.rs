pub mod launcher;
pub mod langauge;
pub mod error;

#[cfg(test)]
mod tests;

pub const LAUNCHER_VERSION: &str = env!("CARGO_PKG_VERSION");