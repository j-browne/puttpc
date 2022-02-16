use puttpc_emu::{
    v2::{Controls, PuttPc},
    Machine,
};

fn main() {
    let mut puttpc = PuttPc::new();

    puttpc.set_input(include_bytes!("v2_test_all_out.bin"));

    println!("{:#?}", puttpc);
    while !puttpc.is_halted() {
        puttpc.step();
        println!("{:#?}", puttpc);
    }
}
