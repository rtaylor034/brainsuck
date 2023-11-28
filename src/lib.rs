
use std::io::{BufReader, BufWriter, Write, Read};
pub mod args;

pub struct State {
    pub memory: Vec<u8>,
    pub cursor: usize,
    pub stored_location: usize,
    pub program_pointer: usize,
}
impl State {
    pub fn new() -> Self {
        Self {
            memory: vec![0],
            cursor: 0,
            stored_location: 0,
            program_pointer: 0,
        }
    }
}
pub enum Instruction {
    Add,
    Sub,
    Right,
    Left,
    Start(usize),
    End(usize),
    Store,
    Goto,
    Write,
    Read,
    Custom(&'static dyn Fn(&mut State)),
}

impl Instruction {
    pub fn perform<R, W>(&self, state: &mut State, read_buf: &mut BufReader<R>, write_buf: &mut BufWriter<W>)
    where R: Read,
          W: Write, {
        let current_cell = state.memory.get_mut(state.cursor)
            .expect(format!("cursor at invalid position?: {}", state.cursor).as_str());

        match self {
            Self::Add => (*current_cell, _) = current_cell.overflowing_add(1),
            Self::Sub => (*current_cell, _) = current_cell.overflowing_sub(1),
            Self::Right => {
                state.cursor += 1;
                if state.cursor >= state.memory.len() {
                    state.memory.push(0);
                }
            }
            Self::Left => state.cursor = state.cursor.checked_sub(1)
                .expect("attempted to reach negative memory index"),
            Self::Start(p) => if *current_cell == 0 {
                state.program_pointer = *p;
            }
            Self::End(p) => if *current_cell != 0 {
                state.program_pointer = *p;
            }
            Self::Store => state.stored_location = state.cursor,
            Self::Goto => state.cursor = state.stored_location,
            Self::Write => drop(write_buf.write(&[*current_cell]).unwrap()),
            Self::Read => drop(read_buf.read(std::slice::from_mut(current_cell)).unwrap()),
            Self::Custom(function) => function(state),
        }
    }
}
pub enum ReadError {
    UnclosedBlock(usize),
    InvalidBlockEnd(usize),
}
impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ReadError::*;
        match self {
            UnclosedBlock(pos) => write!(f, "Unclosed '[' at position {}", pos),
            InvalidBlockEnd(pos) => write!(f, "Dangling ']' at position {}", pos),
        }
    }
}
struct Block(usize, Vec<Instruction>);
pub fn read_code(code_str: &str) -> Result<Vec<Instruction>, ReadError> {
    type CodeBuf = Vec::<Instruction>;
    let mut block_buf = vec![Block(0, CodeBuf::new())];
    let mut pos: usize = 0;
    macro_rules! current_buf { () => { &mut block_buf.last_mut().unwrap().1 } }
    macro_rules! add_instr { ($instruction:expr) => { Ok(current_buf!().push($instruction)) } }
    use Instruction::*;
    for c in code_str.chars() {
        if let Ok(_) = match c {
            '[' => Ok(block_buf.push(Block(pos, CodeBuf::new()))),
            ']' => {
                if block_buf.len() <= 1 {
                    return Err(ReadError::InvalidBlockEnd(pos));
                }
                let Block(start, mut branch) = block_buf.pop().unwrap();
                let root = current_buf!();
                root.push(Instruction::Start(pos));
                root.append(&mut branch);
                root.push(Instruction::End(start));
                Ok(())
            },
            '+' => add_instr!(Add),
            '-' => add_instr!(Sub),
            '>' => add_instr!(Right),
            '<' => add_instr!(Left),
            ':' => add_instr!(Store),
            ';' => add_instr!(Goto),
            '.' => add_instr!(Write),
            ',' => add_instr!(Read),
            // swap
            '|' => add_instr!(Custom(&|state| std::mem::swap(&mut state.stored_location, &mut state.cursor))),
            //debug
            '?' => add_instr!(Custom(&|state| debug_print(state))),
            _ => Err(())
        } {
            pos += 1;
        }
    }
    assert!(!(block_buf.len() < 1));
    if block_buf.len() > 1 {
        return Err(ReadError::UnclosedBlock(block_buf.last().unwrap().0));
    }
    Ok(block_buf.pop().unwrap().1)
}
fn debug_print(state: &State) {
    eprintln!("'?': {}", state.program_pointer);
    for (i, cell) in state.memory.iter().enumerate() {
        let mark = {
            if i == state.cursor { "<" }
            else if i == state.stored_location { ":" }
            else { "" }
        };
        eprintln!( " - {} {}", cell, mark);
    } 
}
