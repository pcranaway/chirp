use std::{fs::File, io::Read};

use vm::VM;

pub mod vm;

fn main() -> anyhow::Result<()> {
    // create vm
    let mut vm = VM::default();

    // read program from cli arguments
    let path = std::env::args().nth(1).expect("no file path given");

    let mut file = File::open(path)?;
    let mut program = Vec::new();
    file.read_to_end(&mut program)?;

    // load program
    vm.load_program(program);

    loop {
        vm.tick()?;
    }
}
