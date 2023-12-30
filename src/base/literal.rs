use std::fmt::Display;

#[derive(Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
}

pub type LiteralValueRef = Box<LiteralValue>;

impl LiteralValue {
    pub fn number(value: f64) -> Self {
        LiteralValue::Number(value)
    }

    pub fn number_ref(value: f64) -> Box<Self> {
        Box::new(LiteralValue::number(value))
    }

    pub fn string(value: String) -> Self {
        LiteralValue::String(value)
    }

    pub fn string_ref(value: String) -> Box<Self> {
        Box::new(LiteralValue::string(value))
    }

    pub fn boolean(value: bool) -> Self {
        LiteralValue::Boolean(value)
    }

    pub fn boolean_ref(value: bool) -> Box<Self> {
        Box::new(LiteralValue::boolean(value))
    }

    pub fn none() -> Self {
        LiteralValue::None
    }

    pub fn none_ref() -> Box<Self> {
        Box::new(LiteralValue::none())
    }
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            LiteralValue::Number(value) => value.to_string(),
            LiteralValue::String(value) => value.to_string(),
            LiteralValue::Boolean(value) => value.to_string(),
            LiteralValue::None => String::from("nil"),
        };

        write!(f, "{}", result)
    }
}
