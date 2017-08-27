use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Add => write!(f, "add"),
            Sub => write!(f, "sub"),
            Neg => write!(f, "neg"),
            Eq => write!(f, "eq"),
            Gt => write!(f, "gt"),
            Lt => write!(f, "lt"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            Not => write!(f, "not"),
        }
    }
}

use self::Op::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Segment {
    Argument,
    Local,
    This,
    That,
    Constant,
    Static,
    Pointer,
    Temp,
}

impl Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Argument => write!(f, "argument"),
            Local => write!(f, "local"),
            This => write!(f, "this"),
            That => write!(f, "that"),
            Constant => write!(f, "constant"),
            Static => write!(f, "static"),
            Pointer => write!(f, "pointer"),
            Temp => write!(f, "temp"),
        }
    }
}

use self::Segment::*;

type Ident<'a> = &'a str;

pub type Label<'a> = Ident<'a>;

pub type FunName<'a> = Ident<'a>;

#[derive(Debug, PartialEq)]
pub enum Inst<'a> {
    Arith { op: Op },
    Pop { segment: Segment, index: u16 },
    Push { segment: Segment, index: u16 },
    DefLabel { label: Label<'a> },
    Goto { label: Label<'a> },
    IfGoto { label: Label<'a> },
    DefFun { name: FunName<'a>, nvars: u16 },
    Call { name: FunName<'a>, nargs: u16 },
    Return,
}

use self::Inst::*;

impl<'a> Display for Inst<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Arith { op } => op.fmt(f),
            Pop { segment, index } => write!(f, "pop {} {}", segment, index),
            Push { segment, index } => write!(f, "push {} {}", segment, index),
            DefLabel { label } => write!(f, "label {}", label),
            Goto { label } => write!(f, "goto {}", label),
            IfGoto { label } => write!(f, "if-goto {}", label),
            DefFun { name, nvars } => write!(f, "function {} {}", name, nvars),
            Call { name, nargs } => write!(f, "call {} {}", name, nargs),
            Return => write!(f, "return"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_fmt {
        ($expected:expr, $actual:expr) => {{
            assert_eq!($expected, format!("{}", $actual));
        }}
    }

    #[test]
    fn display_op() {
        assert_fmt!("add", Add);
        assert_fmt!("sub", Sub);
        assert_fmt!("neg", Neg);
        assert_fmt!("eq", Eq);
        assert_fmt!("gt", Gt);
        assert_fmt!("lt", Lt);
        assert_fmt!("and", And);
        assert_fmt!("or", Or);
        assert_fmt!("not", Not);
    }

    #[test]
    fn display_segment() {
        assert_fmt!("argument", Argument);
        assert_fmt!("local", Local);
        assert_fmt!("this", This);
        assert_fmt!("that", That);
        assert_fmt!("constant", Constant);
        assert_fmt!("static", Static);
        assert_fmt!("pointer", Pointer);
        assert_fmt!("temp", Temp);
    }

    #[test]
    fn display_inst() {
        assert_fmt!("and", Arith { op: And });
        assert_fmt!(
            "pop local 42",
            Pop {
                segment: Local,
                index: 42,
            }
        );
        assert_fmt!(
            "push static 0",
            Push {
                segment: Static,
                index: 0,
            }
        );
        assert_fmt!("label START", DefLabel { label: "START" });
        assert_fmt!("goto LOOP", Goto { label: "LOOP" });
        assert_fmt!("if-goto END", IfGoto { label: "END" });
        assert_fmt!(
            "function Main 3",
            DefFun {
                name: "Main",
                nvars: 3,
            }
        );
        assert_fmt!(
            "call Foo 2",
            Call {
                name: "Foo",
                nargs: 2,
            }
        );
        assert_fmt!("return", Return);
    }
}
