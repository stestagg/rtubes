
use std::io::Error;
use std::path::Path;
use crate::tubes::{Tube, NextResult};
use std::io::Read;
use std::cell::RefCell;
use crate::pump;


pub struct FileReader {
    pub file: RefCell<std::fs::File>,
    pub page_size: usize,
}

pub trait Container<T> {
    fn pages(&self, next: &dyn Tube<[T]>) -> NextResult;
}


impl Container<u8> for FileReader {
    fn pages(&self, next: &dyn Tube<[u8]>) -> NextResult {
        let mut buffer = Vec::with_capacity(self.page_size);
        let mut file = self.file.borrow_mut();
        loop {
            let read_size = file.read(&mut buffer);
            match read_size {
                Ok(size) => {
                    if size == 0 {
                        return NextResult::Continue;
                    }
                    buffer.truncate(size);
                    pump!(next, buffer.as_slice());
                },
                Err(err) => {
                    return NextResult::Error(err)
                },
            }
        }
    }
}


pub struct OpenFile<'a>{
    pub child: Box<dyn Tube<'a, dyn Container<u8> > + 'a>,
    pub page_size: usize,
}

impl<'a, T> Tube<'a, T> for OpenFile<'a> 
    where
T: AsRef<Path>
{
    fn next<'b>(&mut self, value: &'b T) -> NextResult {
        let file = match std::fs::File::open(value){
            Ok(file) => file,
            Err(err) => return NextResult::Error(err),
        };
        let mut reader = FileReader { file: RefCell::new(file), page_size: self.page_size };
        self.child.next(&mut reader)
    }
}