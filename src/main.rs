use lc3_vm::Vm;

use std::env;
use std::fs::File;

fn main() {
    let path_arg = env::args().nth(1).expect("No file path given");

    let mut vm = Vm::new();

    let image_file = File::open(path_arg).expect("Error while opening file");

    vm.load_program(image_file)
        .expect("Error while loading program");

    vm.run();
}
