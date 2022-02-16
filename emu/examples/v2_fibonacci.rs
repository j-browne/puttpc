use puttpc_emu::{v2::PuttPc, Machine};

fn main() {
    let puttpc = PuttPc::new();
    for o in puttpc.into_iter_with_input(include_bytes!("v2_fibonacci.bin")) {
        println!("0x{:02x?}", o);
    }
}
