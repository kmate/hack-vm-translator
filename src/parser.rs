use inst::*;
use nom::{IResult, alpha, alphanumeric, digit, space};
use std::str;
use std::str::FromStr;

use inst::Op::*;
use inst::Segment::*;
use inst::Inst::*;

named!(op<Op>, alt!(
    map!(tag!("add"), |_| Add) |
    map!(tag!("sub"), |_| Sub) |
    map!(tag!("neg"), |_| Neg) |
    map!(tag!("eq"), |_| Eq) |
    map!(tag!("gt"), |_| Gt) |
    map!(tag!("lt"), |_| Lt) |
    map!(tag!("and"), |_| And) |
    map!(tag!("or"), |_| Or) |
    map!(tag!("not"), |_| Not)
));

named!(arith<Inst<'a>>, do_parse!(
    op: op >> (Arith { op: op })
));

named!(segment<Segment>, alt!(
    map!(tag!("argument"), |_| Argument) |
    map!(tag!("local"), |_| Local) |
    map!(tag!("this"), |_| This) |
    map!(tag!("that"), |_| That) |
    map!(tag!("constant"), |_| Constant) |
    map!(tag!("static"), |_| Static) |
    map!(tag!("pointer"), |_| Pointer) |
    map!(tag!("temp"), |_| Temp)
));

named!(index<u16>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(stack_op<Inst<'a>>, do_parse!(
    tag: alt!(tag!("pop") | tag!("push")) >>
    space >>
    segment: segment >>
    space >>
    index: index >>
    (match tag {
        b"pop" => Pop { segment: segment, index: index },
        _ => Push { segment: segment, index: index },
    })
));

named!(ident<&'a str>, map_res!(
    recognize!(do_parse!(
        alpha >>
        many0!(alt!(recognize!(alphanumeric) | recognize!(one_of!("_.$")))) >>
        ()
    )),
    str::from_utf8
));

named!(label_op<Inst<'a>>, do_parse!(
    tag: alt!(tag!("label") | tag!("goto") | tag!("if-goto")) >>
    space >>
    label: ident >>
    (match tag {
        b"label" => DefLabel { label: label },
        b"goto" => Goto { label: label },
        _ => IfGoto { label: label },
    })
));

named!(fun_op<Inst<'a>>, do_parse!(
    tag: alt!(tag!("function") | tag!("call")) >>
    space >>
    name: ident >>
    space >>
    n: index >>
    (match tag {
        b"function" => DefFun { name: name, nvars: n },
        _ => Call { name: name, nargs: n },
    })
));

named!(inst<Inst<'a>>, alt!(
    arith |
    stack_op |
    label_op |
    fun_op |
    do_parse!(tag!("return") >> (Return))
));

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_parse {
        ($expected:expr, $actual:expr) => {{
            assert_eq!(IResult::Done(&b""[..], $expected), $actual);
        }}
    }

    #[test]
    fn parse_artih() {
        assert_parse!(Arith { op: Add }, arith(b"add"));
        assert_parse!(Arith { op: Sub }, arith(b"sub"));
        assert_parse!(Arith { op: Neg }, arith(b"neg"));
        assert_parse!(Arith { op: Eq }, arith(b"eq"));
        assert_parse!(Arith { op: Gt }, arith(b"gt"));
        assert_parse!(Arith { op: Lt }, arith(b"lt"));
        assert_parse!(Arith { op: And }, arith(b"and"));
        assert_parse!(Arith { op: Or }, arith(b"or"));
        assert_parse!(Arith { op: Not }, arith(b"not"));
        assert!(arith(b"something else").is_err());
    }

    #[test]
    fn parse_segment() {
        assert_parse!(Argument, segment(b"argument"));
        assert_parse!(Local, segment(b"local"));
        assert_parse!(This, segment(b"this"));
        assert_parse!(That, segment(b"that"));
        assert_parse!(Constant, segment(b"constant"));
        assert_parse!(Static, segment(b"static"));
        assert_parse!(Pointer, segment(b"pointer"));
        assert_parse!(Temp, segment(b"temp"));
        assert!(segment(b"something else").is_err());
    }

    #[test]
    fn parse_index() {
        assert_parse!(42u16, index(b"42"));
        assert!(index(b"not-a-number").is_err());
        assert!(index(b"123456789").is_err());
    }

    #[test]
    fn parse_stack_op() {
        assert_parse!(Pop { segment: This, index: 20 }, stack_op(b"pop this 20"));
        assert_parse!(Push { segment: That, index: 3 }, stack_op(b"push that 3"));
    }

    #[test]
    fn parse_ident() {
        assert_parse!("some.id$42_start", ident(b"some.id$42_start"));
        assert!(ident(b"-").is_err());
        assert!(ident(b"42apples").is_err());
    }

    #[test]
    fn parse_label_op() {
        assert_parse!(DefLabel { label: "START" }, label_op(b"label START"));
        assert_parse!(Goto { label: "LOOP" }, label_op(b"goto LOOP"));
        assert_parse!(IfGoto { label: "END" }, label_op(b"if-goto END"));
    }

    #[test]
    fn parse_fun_op() {
        assert_parse!(DefFun { name: "Foo", nvars: 3 }, fun_op(b"function Foo 3"));
        assert_parse!(Call { name: "Bar", nargs: 2 }, fun_op(b"call Bar 2"));
    }

    #[test]
    fn parse_inst() {
        assert_parse!(Return, inst(b"return"));
    }
}
