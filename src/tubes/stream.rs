use crate::tubes::{Tube, NextResult};
use crate::pump;
use crate::tubes::file::{Container};

pub struct ReadAll<'a, T> {
    pub child: Box<dyn Tube<'a, [T]> + 'a>,
}

impl<'a, T> Tube<'a, dyn Container<T>> for ReadAll<'a, T> 
    where T: Clone
{
    fn next<'b>(&mut self, value: &dyn Container<T>) -> NextResult {
        //let child = *self.child;
        value.pages(&*self.child)
    }
}
