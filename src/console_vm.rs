use std::{collections::HashSet, fmt};
#[derive(Copy, Clone)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}
impl Instruction {
    pub fn is_nop(&self) -> bool {
        if let Self::Nop(_) = *self {
            true
        } else {
            false
        }
    }
    pub fn is_jmp(&self) -> bool {
        if let Self::Jmp(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn into_nop(&self) -> Self {
        match *self {
            Self::Acc(v) => Self::Nop(v),
            Self::Jmp(v) => Self::Nop(v),
            Self::Nop(v) => Self::Nop(v),
        }
    }

    pub fn into_jmp(&self) -> Self {
        match *self {
            Self::Acc(v) => Self::Jmp(v),
            Self::Jmp(v) => Self::Jmp(v),
            Self::Nop(v) => Self::Jmp(v),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nop(v) => write!(f, "nop {}", v),
            Self::Jmp(v) => write!(f, "jmp {}", v),
            Self::Acc(v) => write!(f, "acc {}", v),
        }
    }
}

pub enum TerminationError {
    CycleDetected,
}

pub struct GameConsoleVM {
    //registers
    pub pc: i32,
    pub acc: i32,
    pub code: Vec<Instruction>,
    int_path: HashSet<i32>,
}

impl GameConsoleVM {
    pub fn new() -> Self {
        Self {
            pc: 0,
            acc: 0,
            code: Vec::new(),
            int_path: HashSet::new(),
        }
    }
    pub fn parse_text_code(&mut self, text: String) {
        text.as_str()
            .lines()
            .map(|line| {
                let field: Vec<_> = line.split(' ').collect();
                match (field[0], field[1]) {
                    ("nop", val) => Instruction::Nop(val.parse().unwrap()),
                    ("jmp", val) => Instruction::Jmp(val.parse().unwrap()),
                    ("acc", val) => Instruction::Acc(val.parse().unwrap()),
                    _ => panic!("unknown instruction! invalid code stream"),
                }
            })
            .for_each(|int| {
                self.code.push(int);
            });
    }

    // returns true when cycle detected
    pub fn run(&mut self) -> Result<(), TerminationError> {
        self.int_path.clear();
        self.pc = 0;
        self.acc = 0;

        while let Some((cur_int, int_pos)) = self.code.get(self.pc as usize).map(|&a| (a, self.pc))
        {
            if let None = self.int_path.get(&int_pos) {
                self.int_path.insert(int_pos);
            } else {
                return Err(TerminationError::CycleDetected);
            }
            match cur_int {
                Instruction::Jmp(val) => {
                    self.pc += val - 1;
                }
                Instruction::Acc(val) => {
                    self.acc += val;
                }
                Instruction::Nop(_) => (),
            }
            self.pc += 1;
        }
        Ok(())
    }
}
impl fmt::Display for GameConsoleVM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, ins) in self.code.iter().enumerate() {
            writeln!(f,"{}:{}",k,ins)?;
        }
        Ok(())
    }
}
