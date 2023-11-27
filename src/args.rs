use std::path::PathBuf;
use gfunc::run::{RunInfo, ValidateExit};
pub struct ProgramInfo {
    pub source_file: PathBuf,
    pub options: Vec<ProgramOption>,
}
pub enum ProgramOption {
    Interactive,
}

impl ProgramInfo {
    pub fn new(run_info: RunInfo) -> Self {
        let arg_file = PathBuf::from(run_info.arguements.validate_exact([|_: &_| true]).auto_exit().pop().unwrap());
        let options: Vec<ProgramOption> = run_info.options.validate([("interactive", Some('i'))]).auto_exit()
            .into_iter().map(|opt| {
                //cant use as_str here???
                match &opt[..] {
                    "interactive" => ProgramOption::Interactive,
                    _ => unreachable!(),
                }
            }).collect();
        let source_file = if arg_file.is_relative() {
            std::env::current_dir()
                .expect("cannot access current working directory, try using an absolute path")
                .join(arg_file)
        } else {
            arg_file
        };
        ProgramInfo {
            source_file,
            options,
        }
    }
}
