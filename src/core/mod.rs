


mod done;
mod progress;

/// A parser will return `Progress` or stop short and give `Fail`.
pub type Result = std::result::Result<Progress, Done>;

pub use progress::Progress;
pub use done::Done;