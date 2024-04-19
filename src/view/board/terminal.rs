use mockall::automock;
use terminal_size::terminal_size;
pub use terminal_size::{Height, Width};

#[automock]
pub trait TerminalSizeAcquisitor {
    fn get(&self) -> Option<(Width, Height)> {
        terminal_size()
    }
}
pub struct Terminal {}
impl TerminalSizeAcquisitor for Terminal {}
