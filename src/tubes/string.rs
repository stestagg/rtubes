use crate::tubes::{Tube, NextResult};
use std::borrow::Cow;

pub struct GetItem<'a, T>
{
    pub child: Box<dyn Tube<'a, T>>,
    pub index: usize
}


impl<'a, T> Tube<'a, [T]> for GetItem<'a, T>
{
    fn next<'b>(&mut self, value: &'b [T]) -> NextResult {
        let child = &mut self.child;
        let val = &value[self.index];
        child.next(val)
    }
}

pub struct AsString<'a>{
    pub child: Box<dyn Tube<'a, str> + 'a>,
}
impl<'a> Tube<'a, [u8]> for AsString<'a> {
    fn next<'b>(&mut self, value: &'b [u8]) -> NextResult
        {
        let cowval: Cow<'b, str> = String::from_utf8_lossy(value);
        let strval = &*cowval;
        let result = self.child.next(strval);
        result
    }
}