pub trait Weight {
    /// Вес операции. Определяет приоритет операций между друг другом.
    /// Операции с наибольшим весом имеют наибольший приоритет.
    fn weight(&self) -> u8;
}

#[derive(Clone)]
pub enum Func {
    Sin,
    Cos,
    Tg,
    Ctg,
    Sqrt,
}

impl Weight for Func {
    fn weight(&self) -> u8 { 4 }
}

impl std::fmt::Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Func::Sin => { "sin" }
                Func::Cos => { "cos" }
                Func::Tg => { "tg" }
                Func::Ctg => { "ctg" }
                Func::Sqrt => { "√" }
            }
        )
    }
}

impl TryFrom<&str> for Func {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "sin" => Ok(Func::Sin),
            "cos" => Ok(Func::Cos),
            "tg" => Ok(Func::Tg),
            "ctg" => Ok(Func::Ctg),
            "√" => Ok(Func::Sqrt),
            _ => Err(())
        }
    }
}

/// Алгебраические операции.
#[derive(Clone)]
pub enum Op {
    /// Сложение - соответствует знаку '+'.
    Add,
    /// Вычитание - соответствует знаку '-'.
    Sub,
    /// Умножение - соответствует знаку '*'.
    Multi,
    /// Деление - соответствует знаку '/'.
    Div,
    /// Возведение в степень - соответствует знаку '^'.
    Exp,
    /// Символы ограничения области вычислений.
    ParenLeft,
    ParenRight,
}

impl Weight for Op {
    fn weight(&self) -> u8 {
        match self {
            Op::Add | Op::Sub => { 1 }
            Op::Multi | Op::Div => { 2 }
            Op::Exp => { 3 }
            Op::ParenRight | Op::ParenLeft => { 0 }
        }
    }
}


impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Add => { "+" }
                Op::Sub => { "-" }
                Op::Multi => { "*" }
                Op::Div => { "/" }
                Op::Exp => { "^" }
                Op::ParenLeft => { "(" }
                Op::ParenRight => { ")" }
            }
        )
    }
}


impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "^" => Ok(Op::Exp),
            "/" => Ok(Op::Div),
            "*" => Ok(Op::Multi),
            "-" => Ok(Op::Sub),
            "+" => Ok(Op::Add),
            ")" => Ok(Op::ParenRight),
            "(" => Ok(Op::ParenLeft),
            _ => Err(())
        }
    }
}

#[derive(Clone)]
pub enum Token {
    /// Одинарные функции.
    Function(Func),
    /// Операции над числами.
    Operation(Op),
    /// Число (вещественное).
    Operand(f64),
}

impl TryFrom<&str> for Token {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(o) = Op::try_from(s) {
            Ok(Token::Operation(o))
        } else if let Ok(f) = Func::try_from(s) {
            Ok(Token::Function(f))
        } else if let Ok(val) = s.parse::<f64>() {
            if val.is_infinite() {
                Err(())
            } else { Ok(Token::Operand(val)) }
        } else { Err(()) }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Function(func) => { func.to_string() }
                Token::Operation(op) => { op.to_string() }
                Token::Operand(o) => { o.to_string() }
            }
        )
    }
}