use std::rc::Rc;

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

pub enum BinaryOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub enum UnaryOperation {
    ArithmeticNegation,
}

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
