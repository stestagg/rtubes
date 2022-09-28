use crate::tubes::{Tube, InputTube, NextResult};
use crate::pump;
use std::io::Error;


pub struct Counter<'a, T>
{
    pub child: Box<dyn Tube<'a, T> + 'a>,
    pub start_value: T,
}


impl<'a, T> InputTube for Counter<'a, T> 
where
    T: std::ops::AddAssign<T> + 'a,
    T: std::convert::From<u8>,
    T: Copy
{
    fn go(&mut self) -> Option<Error> {
        let increment: T = T::from(1 as u8);
        let mut value = self.start_value;
        loop{
            let child = &mut self.child;
            match child.next(&value) {
                NextResult::Continue => {},
                NextResult::Stop => {
                    return None;
                }
                NextResult::Error(err) => {
                    return Some(err);
                }
            }
            value += increment;
        }
    }
}


pub struct Each<'a, T>
{
    pub child: Box<dyn Tube<'a, T> + 'a>,
    pub iter: Option<Box<dyn Iterator<Item=T>>>,
}
impl<'a, 'b, T: 'a> InputTube for Each<'a, T> 
    {
    fn go(&mut self) -> Option<Error> {    

        let iter = match self.iter {
            Some(ref mut iter) => iter,
            None => return None,
        };

        loop {
            let val = match iter.next() {
                Some(val) => val,
                None => return None,
            };
            match self.child.next(&val) {
                NextResult::Continue => {
                },
                NextResult::Stop => {
                    return None;
                }
                NextResult::Error(err) => {
                    return Some(err);
                }
            }
        }
    }
}