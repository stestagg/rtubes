mod tubes;
use crate::tubes::simple::{One, Skip};
use crate::tubes::input::{Each};
use crate::tubes::{InputTube};
use crate::tubes::file::{OpenFile};
use crate::tubes::stream::{ReadAll};
use crate::tubes::string::{AsString};

use std::cell::RefCell;

fn main() {

    //let mut result = None;
    let vals = vec!["temp/test.txt", "temp/two.txt"];
    let vals_iter = vals.into_iter();
    let r2 = RefCell::new(String::from("HI"));
    {
        let out = One{dest: r2};
        let asstr = AsString{ child: Box::new(out) };
        let content = ReadAll{ child: Box::new(asstr) };
        let file = OpenFile{ child: Box::new(content), page_size: 1024 };
        let skip = Skip{child: Box::new(file), num: 1};

        Each{iter: Some(Box::new(vals_iter)), child: Box::new(skip)}.go();
    }

    dbg!(r2.borrow());

    //println!("Hello, world!");
}
