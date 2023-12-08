#![macro_use]

mod filter;
mod map;
mod optional;
mod then;

pub use filter::*;
pub use map::*;
pub use optional::*;
pub use then::*;

macro_rules! select {
    ($($cs:expr),+) => {
        |input: &mut Input<'src>| {
            _select_inner! { input => $($cs),+ }

            Ok(None)
        }
    };
}

macro_rules! _select_inner {
    ($i:expr => $c:expr) => {
        if let Some(r) = Parser::parse(&$c, $i)? {
            return Ok(Some(r));
        }
    };

    ($i:expr => $c:expr, $($cs:expr),+) => {
        _select_inner! { $i => $c }
        _select_inner! { $i => $($cs),+ }
    };
}
