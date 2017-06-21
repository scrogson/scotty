extern crate scotty;

use scotty::vm::VM;

fn main() {
    let vm = VM::new();

    vm.start()
}
