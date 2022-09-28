use crate::tubes::{Tube, NextResult};
use std::cell::RefCell;

pub struct One<T> {
    pub dest: RefCell<T>
}

impl<'a, T, U> Tube<'a, T> for One<U> 
where T: ?Sized + 'a,
      &'a T: Into<U>

{
    fn next<'b>(&mut self, value: &'b T) -> NextResult {
        let uval = value.into();
        self.dest.replace(uval);
        NextResult::Stop
    }
}

pub struct Head<'a, T: ?Sized>{
    pub child: Box<dyn Tube<'a, T> + 'a>,
    pub num: usize
}
impl<'a, T> Tube<'a, T> for Head<'a, T> {
    fn next<'b>(&mut self, value: &'b T) -> NextResult {
        if self.num == 0{
            NextResult::Stop
        } else {
            self.num -= 1;
            self.child.next(value)
        }
    }
}

pub struct Skip<'a, T: ?Sized>{
    pub child: Box<dyn Tube<'a, T> + 'a>,
    pub num: usize
}
impl<'a, T: ?Sized> Tube<'a, T> for Skip<'a, T>
{
    fn next<'b>(&mut self, value: &'b T) -> NextResult {
        if self.num == 0 {
            self.child.next(value)
        } else {
            self.num -= 1;
            NextResult::Continue
        }
    }
}


pub struct Repeat<'a, T: ?Sized>{
    pub child: Box<dyn Tube<'a, T> + 'a>,
    pub num: usize
}
impl<'a, T> Tube<'a, T> for Repeat<'a, T> {
    fn next<'b>(&mut self, value: &'b T) -> NextResult {
        if self.num == 0 {
            NextResult::Stop
        } else {
            self.num -= 1;
            self.child.next(value)
        }
    }
}
