// ©2025 - BestMat - All rights reserved.

mod vm;
use vm::RustVM;

fn main() {
    println!("Hello, world!");

    let mut rustvm = RustVM::new();
    let pointer1 = rustvm.alloc("00000001");
    let pointer2 = rustvm.alloc("00000010");
    let pointer3 = rustvm.alloc("00000001");
    rustvm.free(pointer2);
    let pointer4 = rustvm.alloc("11111111");

    println!("{:#?}", rustvm);
}
