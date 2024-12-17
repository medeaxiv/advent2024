pub struct Vm<T: Trace = ()> {
    registers: [i64; 3],
    ip: i64,
    trace: T,
}

impl Vm<()> {
    pub fn new() -> Self {
        Self {
            registers: [0; 3],
            ip: 0,
            trace: (),
        }
    }

    #[allow(dead_code)]
    pub fn tracing(self) -> Vm<Vec<TraceEntry>> {
        Vm {
            registers: self.registers,
            ip: self.ip,
            trace: Vec::new(),
        }
    }
}

impl Vm<Vec<TraceEntry>> {
    #[allow(dead_code)]
    pub fn get_trace(&self) -> &[TraceEntry] {
        &self.trace
    }
}

impl<T: Trace> Vm<T> {
    pub fn set_registers(&mut self, registers: [i64; 3]) {
        self.registers = registers;
    }

    pub fn reset(&mut self) {
        self.registers = [0; 3];
        self.ip = 0;
        self.trace.clear();
    }

    pub fn execute_program(&mut self, program: &[i64]) -> Result<Vec<i64>, VmError> {
        let mut output = Vec::new();

        loop {
            let result = self
                .decode(program)
                .and_then(|instruction| self.execute(instruction));

            match result {
                Ok(ExecuteResult::None) => {}
                Ok(ExecuteResult::Output(value)) => output.push(value),
                Err(VmError::Halt) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(output)
    }

    fn fetch(&self, program: &[i64], offset: i64) -> Result<i64, VmError> {
        let address = self.ip + offset;

        if address >= 0 && address < program.len() as i64 {
            Ok(program[address as usize])
        } else {
            Err(VmError::FetchOutOfBounds)
        }
    }

    fn decode(&self, program: &[i64]) -> Result<Instruction, VmError> {
        let opcode = self.fetch(program, 0).map_err(|_| VmError::Halt)?;
        let operand = self
            .fetch(program, 1)
            .map_err(|_| VmError::MissingOperand)?;

        match opcode {
            0 => Ok(Instruction::Adv(operand)),
            1 => Ok(Instruction::Bxl(operand)),
            2 => Ok(Instruction::Bst(operand)),
            3 => Ok(Instruction::Jnz(operand)),
            4 => Ok(Instruction::Bxc(operand)),
            5 => Ok(Instruction::Out(operand)),
            6 => Ok(Instruction::Bdv(operand)),
            7 => Ok(Instruction::Cdv(operand)),
            _ => Err(VmError::UnknownOpcode(opcode)),
        }
    }

    fn execute(&mut self, instruction: Instruction) -> Result<ExecuteResult, VmError> {
        let mut advance_ip = true;
        let mut result = ExecuteResult::None;

        let mut trace_entry = self.new_trace_entry(instruction);

        match instruction {
            Instruction::Adv(operand) => {
                let operand = self.combo_operand_value(operand)?;
                let denominator = 1 << operand;
                self.registers[0] = self.registers[0] / denominator;
            }
            Instruction::Bdv(operand) => {
                let operand = self.combo_operand_value(operand)?;
                let denominator = 1 << operand;
                self.registers[1] = self.registers[0] / denominator;
            }
            Instruction::Cdv(operand) => {
                let operand = self.combo_operand_value(operand)?;
                let denominator = 1 << operand;
                self.registers[2] = self.registers[0] / denominator;
            }
            Instruction::Bxl(operand) => {
                self.registers[1] = self.registers[1] ^ operand;
            }
            Instruction::Bst(operand) => {
                let operand = self.combo_operand_value(operand)?;
                self.registers[1] = operand & 0b0111;
            }
            Instruction::Jnz(operand) => {
                let jump = self.registers[0] != 0;
                trace_entry.jump = Some(jump);
                if jump {
                    self.ip = operand;
                    advance_ip = false;
                }
            }
            Instruction::Bxc(_operand) => {
                self.registers[1] = self.registers[1] ^ self.registers[2];
            }
            Instruction::Out(operand) => {
                let operand = self.combo_operand_value(operand)?;
                let output = operand & 0b0111;
                trace_entry.output = Some(output);
                result = ExecuteResult::Output(output);
            }
        }

        self.trace.push(trace_entry);

        if advance_ip {
            self.ip += instruction.len() as i64;
        }

        Ok(result)
    }

    fn combo_operand_value(&self, operand: i64) -> Result<i64, VmError> {
        match operand {
            0 | 1 | 2 | 3 => Ok(operand),
            4 => Ok(self.registers[0]),
            5 => Ok(self.registers[1]),
            6 => Ok(self.registers[2]),
            _ => Err(VmError::InvalidComboOperand(operand)),
        }
    }

    #[inline]
    fn new_trace_entry(&self, instruction: Instruction) -> TraceEntry {
        TraceEntry {
            instruction,
            registers: self.registers,
            ip: self.ip,
            output: None,
            jump: None,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum VmError {
    #[error("Virtual machine halted")]
    Halt,
    #[error("Unknown opcode {0}")]
    UnknownOpcode(i64),
    #[error("Missing operand")]
    MissingOperand,
    #[error("Fetch out of program bounds")]
    FetchOutOfBounds,
    #[error("Invalid combo operand {0}")]
    InvalidComboOperand(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Adv(i64),
    Bxl(i64),
    Bst(i64),
    Jnz(i64),
    Bxc(i64),
    Out(i64),
    Bdv(i64),
    Cdv(i64),
}

impl Instruction {
    pub const fn len(&self) -> usize {
        match self {
            Self::Adv(..) => 2,
            Self::Bxl(..) => 2,
            Self::Bst(..) => 2,
            Self::Jnz(..) => 2,
            Self::Bxc(..) => 2,
            Self::Out(..) => 2,
            Self::Bdv(..) => 2,
            Self::Cdv(..) => 2,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Adv(operand) => write!(f, "adv {operand:o}"),
            Self::Bxl(operand) => write!(f, "bxl {operand:o}"),
            Self::Bst(operand) => write!(f, "bst {operand:o}"),
            Self::Jnz(operand) => write!(f, "jnz {operand:o}"),
            Self::Bxc(operand) => write!(f, "bxc {operand:o}"),
            Self::Out(operand) => write!(f, "out {operand:o}"),
            Self::Bdv(operand) => write!(f, "bdv {operand:o}"),
            Self::Cdv(operand) => write!(f, "cdv {operand:o}"),
        }
    }
}

enum ExecuteResult {
    None,
    Output(i64),
}

pub trait Trace {
    fn push(&mut self, entry: TraceEntry);
    fn clear(&mut self);
}

impl Trace for () {
    #[inline]
    fn push(&mut self, _: TraceEntry) {}

    #[inline]
    fn clear(&mut self) {}
}

impl Trace for Vec<TraceEntry> {
    #[inline]
    fn push(&mut self, entry: TraceEntry) {
        self.push(entry);
    }

    #[inline]
    fn clear(&mut self) {
        Vec::clear(self);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TraceEntry {
    pub instruction: Instruction,
    pub registers: [i64; 3],
    pub ip: i64,
    pub output: Option<i64>,
    pub jump: Option<bool>,
}

impl std::fmt::Display for TraceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  {}; [IP={:>2x}] [A={:>16o}] [B={:>16o}] [C={:>16o}]",
            self.instruction, self.ip, self.registers[0], self.registers[1], self.registers[2]
        )?;

        if let Some(output) = self.output {
            write!(f, " [out={output:o}]")?;
        }

        if let Some(jump) = self.jump {
            write!(f, " [jump={jump}]")?;
        }

        Ok(())
    }
}
