// ©2025 - BestMat - All rights reserved.

mod vm;
use vm::RustVM;

fn main() {
    println!("Hello, world!");

    let mut rustvm = RustVM::new();
    let pointer1 = rustvm.memalloc(vec!["00000001"], 1);
    let pointer2 = rustvm.memalloc(vec!["00000010"], 1);
    rustvm.memfree(pointer2);
    rustvm.regmov("r1", "11111111");
    println!("{:#?}", rustvm.memget(pointer1));
    println!("{:#?}", rustvm.regget("r1"));
    rustvm.intalloc(21 as i8, 8);
    rustvm.stralloc("Hello", "5");

    println!("{:#?}", rustvm);
}
