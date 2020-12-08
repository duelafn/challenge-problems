
type Word = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Acc(Word),
    Jmp(Word),
    Nop(Word),
    Halt,
}
impl Instruction {
    pub fn new_from(src: &str, _version: usize) -> Result<Instruction, String> {
        use Instruction::*;
        let mut iter = src.split(" ");
        let code = iter.next().ok_or_else(|| "Can't find instruction".to_string())?;
        let num  = iter.next().ok_or_else(|| "Can't find argument".to_string())?.parse().or_else(|err| Err(format!("Can't parse number in {}: {}", src, err)))?;
        match code {
            "jmp" => Ok(Jmp(num)),
            "acc" => Ok(Acc(num)),
            "nop" => Ok(Nop(num)),
            _     => Err(format!("Unknown instruction: {}", code)),
        }
    }
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        match self {
            Acc(n) => write!(f, "acc {}", n),
            Jmp(n) => write!(f, "jmp {}", n),
            Nop(n) => write!(f, "nop {}", n),
            Halt   => write!(f, "HALT"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InterpreterState {
    pub accumulator: Word,
    pub index: usize,
}
impl InterpreterState {
    pub fn new() -> InterpreterState {
        InterpreterState { accumulator: 0, index: 0 }
    }

    #[inline]
    pub fn operate(&mut self, instr: &Instruction) {
        use Instruction::*;
        match instr {
            Acc(n) => { self.accumulator += *n; self.index += 1; },
            Jmp(n) => {
                if *n >= 0 { self.index += *n as usize; }
                else if self.index >= (n.abs() as usize) { self.index -= n.abs() as usize; }
                else { panic!("invalid jump!"); }
            },
            Nop(_) => { self.index += 1; },
            Halt   => { },
        }
    }
}

#[derive(Debug)]
pub struct Interpreter {
    program: Vec<Instruction>,
}
impl Interpreter {
    #[inline]
    pub fn step(&self, state: &mut InterpreterState) {
        if let Some(instr) = self.program.get(state.index) {
            state.operate(instr);
        }
    }

    #[inline]
    pub fn run(&self, state: &mut InterpreterState) {
        while !self.is_halted(state) {
            self.step(state);
        }
    }

    #[inline]
    pub fn is_halted(&self, state: &InterpreterState) -> bool {
        match self.program.get(state.index) {
            None | Some(Instruction::Halt) => true,
            _ => false,
        }
    }
}
impl std::convert::TryFrom<&str> for Interpreter {
    type Error = String;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let mut program = Vec::new();
        for (lineno, chunk) in src.lines().enumerate() {
            program.push(Instruction::new_from(chunk, 1).or_else(|err| Err(format!("Parse error at '{}', line {}: {}", chunk, lineno+1, err)))?);
        }
        return Ok(Interpreter { program });
    }
}
impl std::fmt::Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instr in self.program.iter() {
            write!(f, "{}\n", instr)?
        }
        Ok(())
    }
}
impl std::ops::Deref for Interpreter {
    type Target = Vec<Instruction>;
    fn deref(&self) -> &Self::Target { &self.program }
}
