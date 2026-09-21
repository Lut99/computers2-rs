//  AST.rs
//    by Lut99
//
//  Description:
//!   Defines the CAS1 assembly language AST.
//


/// The toplevel representation of a CAS1 file.
pub struct Program {
    /// The list of directives in the program.
    pub direct: Vec<Directive>,
}

/// Defines a directive, i.e., a statement.
pub enum Directive {
    /// It's an instruction.
    Instr(Instr),
    /// It's arbitrary data.
    Data(Vec<Lit>),
    /// It's a label.
    Label(Label),
}



/// Represents an instruction.
pub struct Instr {
    /// The type of instruction.
    pub ty:   Ident,
    /// The arguments to the instruction.
    pub args: Vec<InstrArg>,
}

/// Represents the argument to an instruction.
pub enum InstrArg {
    /// It's a literal value.
    Lit(Lit),
    /// It's a register value.
    Reg(Reg),
    /// It's a label.
    Label(Label),
}



/// Represents a label.
pub struct Label {
    /// The dot token introducing the label.
    pub dot_token: Dot,
    /// The identifier naming it.
    pub ident:     Ident,
}

/// Represents a register.
pub struct Reg {
    /// The hashtag-token introducing the register.
    pub hash_token: Hash,
    /// The register identifier naming it.
    pub regident:   RegIdent,
}
pub enum RegIdent {
    Ident(Ident),
    Int(LitInt),
}



/// Represents an identifier.
pub struct Ident {
    /// The value of the identifier.
    pub value: String,
}

/// Represents a literal value.
pub enum Lit {
    /// It's a decimal/hexadecimal/binary, unsigned integer value.
    Int(LitInt),
    /// It's a decimal, signed integer value. That means negative!
    NegInt(LitNegInt),
    /// It's a decimal, floating-point value.
    Float(LitFloat),
}

pub struct LitInt {
    /// The value of the literal.
    pub value: u64,
}
pub struct LitNegInt {
    /// The value of the literal.
    pub value: i64,
}
pub struct LitFloat {
    /// The value of the literal.
    pub value: f64,
}



pub struct Dot;
pub struct Hash;
