pub mod simple;
pub mod string;
pub mod input;
pub mod file;
pub mod stream;

use std::io::Error;

#[macro_export]
macro_rules! pump {
    ($child: expr, $value: expr) => {
        match $child.next($value) {
            NextResult::Continue => {
            },
            NextResult::Stop => {
                return NextResult::Stop;
            }
            NextResult::Error(err) => {
                return NextResult::Error(err);
            }
        }
    };
}


pub enum NextResult{
    Continue,
    Stop,
    Error(Error)
}


pub trait Tube<'a, U: ?Sized> {
    fn next<'b>(&mut self, value: &'b U) -> NextResult;
}


pub trait InputTube {
    fn go(&mut self) -> Option<Error>;
}

