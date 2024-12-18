#[derive(Debug,PartialEq)]
pub enum Opcode{
    HLT,
    IGL, // illegal
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPB,
    JMPF,
    EQ,
    NEQ,
    GT,
    LT,
    GTQ,
    LTQ,
    JEQ
}

#[derive(Debug,PartialEq)]
pub struct Instruction{
    opcode: Opcode
}

impl Instruction{
    pub fn new(opcode: Opcode) -> Instruction{
        Instruction {opcode}
    }
}

impl From<u8> for Opcode{
    fn from(v: u8)-> Self{
        match v {
        0 => Opcode::HLT,
        1 => Opcode::LOAD,
        2 => Opcode::ADD,
        3 => Opcode::SUB,
        4 => Opcode::MUL,
        5 => Opcode::DIV,
        6 => Opcode::JMP,
        7 => Opcode::JMPF,
        8 => Opcode::JMPB,
        9 => Opcode::EQ,
        10 => Opcode::NEQ,
        11 => Opcode::GT,
        12 => Opcode::LT,
        13 => Opcode::GTQ,
        14 => Opcode::LTQ,
        15 => Opcode::JEQ,
        _ => Opcode::IGL
        }
    }
}




#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn make_instruction() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode,Opcode::HLT );
    }

    #[test]
    fn create_instruction() {
        let instruct = Instruction::new(Opcode::HLT);
        assert_eq!(instruct.opcode,Opcode::HLT);
    }
}
