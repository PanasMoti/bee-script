#![allow(unused)]

mod interperter;
mod stack;
use crate::interperter::*;
use std::cell::RefCell;

thread_local! {
    static INTERPERTER:RefCell<Interperter> = RefCell::new(Interperter::new());
}

pub fn exec_code(code:String) {
    INTERPERTER.with(|inter| inter.borrow_mut().exec(code));
}

pub fn exec_file(file:String) {
    let code = std::fs::read_to_string(file).unwrap();
    exec_code(code);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interperter::*;

    #[test]
    fn test1() {
        println!("{:?}",Commands::BARRY);
    }
    #[test]
    fn test2() {
        println!("{:?}",Interperter::new());
    }
    #[test]
    fn test3() {
        let mut v = vec![1,2,3,4];
        let l = v.len();
        v.swap(0,l-1);
        println!("{:?}",v);
    }

    #[test]
    fn test4() {
        let mut i = Interperter::new();
        let code = String::from("YELLOW\nBEE\nBLACK\nFLY 1\n");
        i.exec(code);
        // println!("{:?}",i);
    }
    #[test]
    fn test5() {
        let mut i = Interperter::new();
        let code = std::fs::read_to_string("testing/bee.bs").unwrap();
        i.exec(code);

    }
    

}
