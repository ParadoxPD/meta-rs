mod compiler;
mod instr;
mod interpreter;
mod op;

use std::{fs, io::Write, path, process::exit};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    compile: bool,
    #[arg(long, default_value_t = false)]
    exec: bool,
    #[arg(long, default_value_t = false)]
    optimize: bool,
    #[arg(long, default_value_t = false)]
    decompile: bool,
    file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut bytecode = vec![];

    if args.compile {
        if path::Path::new(&args.file).extension().is_some()
            && path::Path::new(&args.file).extension().unwrap() == "metabin"
        {
            eprint!("Error: metabin file supplied, refusing to compile bytecode. Did you mean to execute it?\n");
            exit(1);
        }
        bytecode = match compiler::compile(&fs::read_to_string(&args.file)?) {
            Err(err) => {
                eprintln!("Error compiling '{}':'{}'", args.file, err);
                exit(1);
            }
            Ok(val) => val,
        }
    }

    if args.exec {
        if !args.compile {
            if path::Path::new(&args.file).extension().is_some()
                && path::Path::new(&args.file).extension().unwrap() == "meta"
            {
                eprint!("Error: meta file supplied, failed to execute meta file. Did you mean to compile it?\n");
                exit(1);
            }
            bytecode = interpreter::decode_instructions(fs::read(&args.file)?)?;
        }
        let mut vm = interpreter::VM::new();
        vm.execute(bytecode)?;
    } else {
        let mut new_file: fs::File =
            fs::File::create(path::Path::new(&args.file).with_extension("metabin"))?;
        new_file.write_all(interpreter::encode_instructions(&bytecode[0..])?.as_slice())?;
    }

    return Ok(());
}
