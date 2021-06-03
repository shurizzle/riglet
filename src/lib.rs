mod figure;
mod line;
mod utils;

pub use crate::figure::FIGure;
pub use crate::line::FIGline;

pub use figfont::*;

mod prelude {
    pub use super::error::Error;
    pub use super::result::Result;
    pub use super::FIGfont;
    pub use super::FIGure;
}
