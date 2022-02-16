use clap::{ArgEnum, Parser};
use std::{io, fs, path::PathBuf};
use puttpc_emu::{Machine, v1, v2};

#[derive(Debug, Parser)]
#[clap(name = "PuttPc Emulator", about, long_about = None)]
struct Cli {
    /// The version of PuttPc to emulate
    #[clap(short, long, arg_enum, default_value_t)]
    version: Version,

    /// Suppress printing of output
    #[clap(long)]
    no_output: bool,

    /// Print state after each step
    #[clap(long)]
    state: bool,

    /// Pause after each step
    #[clap(long)]
    pause: bool,

    /// The input to feed into the computer
    input: PathBuf,

    // TODO: output format
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Version {
    V1,
    V2,
    V3,
}

impl Default for Version {
    fn default() -> Self {
        Self::V3
    }
}

fn main() {
    let cli = Cli::parse();

    //TODO: fail gracefully
    let input = fs::read(&cli.input).unwrap();

    match cli.version {
        Version::V1 => run(v1::PuttPc::with_input(&input), cli),
        Version::V2 => run(v2::PuttPc::with_input(&input), cli),
        Version::V3 => todo!(),//run(v3::PuttPc::with_input(&input), cli),
    }
}

fn run(mut machine: impl Machine, cli: Cli) {
    // a buffer for stdin.read_line. data isn't used
    let mut s = String::new();

    while !machine.is_halted() {
        let out = machine.step();

        if cli.state {
            todo!("print state");
        }

        if let Some(out) = out {
            if !cli.no_output {
                println!("Output: 0x{:02x}", out);
            }
        }

        if cli.pause {
            println!("Press Enter to continue");
            // TODO: do this better
            io::stdin().read_line(&mut s).unwrap();
        }
    }
}
