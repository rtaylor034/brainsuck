use brainsuck::*;
use std::io::{BufWriter, BufReader};
use std::io;
use gfunc::run::RunInfo;
fn main() -> std::io::Result<()> {
    let program_info = args::ProgramInfo::new(RunInfo::get_from_env());

    let instructions = match read_code(std::fs::read_to_string(program_info.source_file)?.as_str()) {
        Ok(v) => v,
        Err(e) => { eprintln!("{}", e); std::process::exit(1) },
    };
    let mut program_state = State::new();
    let mut stdin = BufReader::new(io::stdin());
    let mut stdout = BufWriter::new(io::stdout());
    let eof = instructions.len();
    while program_state.program_pointer < eof {
        instructions[program_state.program_pointer].perform(&mut program_state, &mut stdin, &mut stdout);
        program_state.program_pointer += 1;
    }
    Ok(())
}
