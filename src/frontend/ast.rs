use std::{fmt, rc::Rc};

#[derive(Debug)]
pub enum Value {
    SingleFloat(f32),
    DoubleFloat(f64),

    Long(i64),
    UnsignedLong(u64),

    Integer(i32),
    UnsignedInteger(u32),

    Short(i16),
    UnsignedShort(u16),

    Byte(u8),
    SignedByte(i8),
}

#[derive(strum_macros::Display, Debug)]
pub enum BinaryOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(strum_macros::Display, Debug)]
pub enum UnaryOperation {
    ArithmeticNegation,
}

#[derive(Debug)]
pub enum ValueNode {
    Literal(Value),
    UnaryExpression {
        opr: UnaryOperation,
        value: Rc<ValueNode>,
    },
    BinaryExpression {
        left: Rc<ValueNode>,
        opr: BinaryOperation,
        right: Rc<ValueNode>,
    },
}

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ValueNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
