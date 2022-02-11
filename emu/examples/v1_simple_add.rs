use puttpc_emu::{v1::PuttPc, Machine};

fn main() {
    let mut puttpc = PuttPc::new();
    let output = puttpc.run_with_input(include_bytes!("v1_simple_add.bin"));
    for o in output {
        println!("0x{:x?}", o);
    }
}
