// IOS Colors 1,2,3,4,5,6,7
// Extended Colors 0-255
// RGB Colors

#[derive(Debug, Hash,PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Clone)]
pub struct Background(Value);

impl std::fmt::Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}

impl Background {
    pub fn black(b: bool) -> Self {Self(Value::Iso { color: Iso::Black, bright: b })}
    pub fn red(b: bool) -> Self {Self(Value::Iso { color: Iso::Red, bright: b })}
    pub fn green(b: bool) -> Self {Self(Value::Iso { color: Iso::Green, bright: b })}
    pub fn yellow(b: bool) -> Self {Self(Value::Iso { color: Iso::Yellow, bright: b })}
    pub fn blue(b: bool) -> Self {Self(Value::Iso { color: Iso::Blue, bright: b })}
    pub fn magenta(b: bool) -> Self {Self(Value::Iso { color: Iso::Magenta, bright: b })}
    pub fn cyan(b: bool) -> Self {Self(Value::Iso { color: Iso::Cyan, bright: b })}
    pub fn white(b: bool) -> Self {Self(Value::Iso { color: Iso::White, bright: b })}

    pub fn iso(iso: Iso, bright: bool) -> Self {
        Self(Value::Iso { color: iso, bright: bright })
    }

    pub fn extended(ext: u8) -> Self {
        Self(Value::Extended(ext))
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(Value::Rgb { r: r, g: g, b: b })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Clone)]
pub struct Foreground(Value);

impl Foreground {
    pub fn black(b: bool) -> Self {Self(Value::Iso { color: Iso::Black, bright: b })}
    pub fn red(b: bool) -> Self {Self(Value::Iso { color: Iso::Red, bright: b })}
    pub fn green(b: bool) -> Self {Self(Value::Iso { color: Iso::Green, bright: b })}
    pub fn yellow(b: bool) -> Self {Self(Value::Iso { color: Iso::Yellow, bright: b })}
    pub fn blue(b: bool) -> Self {Self(Value::Iso { color: Iso::Blue, bright: b })}
    pub fn magenta(b: bool) -> Self {Self(Value::Iso { color: Iso::Magenta, bright: b })}
    pub fn cyan(b: bool) -> Self {Self(Value::Iso { color: Iso::Cyan, bright: b })}
    pub fn white(b: bool) -> Self {Self(Value::Iso { color: Iso::White, bright: b })}

    pub fn iso(iso: Iso, bright: bool) -> Self {
        Self(Value::Iso { color: iso, bright: bright })
    }

    pub fn extended(ext: u8) -> Self {
        Self(Value::Extended(ext))
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(Value::Rgb { r: r, g: g, b: b })
    }
}

impl std::fmt::Display for Foreground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ansi())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Clone)]
pub enum Value {
    Iso { color: Iso, bright: bool },
    Extended(u8),
    Rgb { r: u8, g: u8, b: u8 },
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Clone)]
pub enum Iso {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Iso {
    pub fn to_char(&self) -> char {
        match self {
            Iso::Black => '0',
            Iso::Red => '1',
            Iso::Green => '2',
            Iso::Yellow => '3',
            Iso::Blue => '4',
            Iso::Magenta => '5',
            Iso::Cyan => '6',
            Iso::White => '7',
        }
    }
}

impl Foreground {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn to_ansi(&self) -> String {
        match &self.0 {
            Value::Iso { color, bright } => {
                format!("\x1b[{}{}m", if *bright { 9 } else { 3 }, color.to_char())
            }
            Value::Extended(val) => format!("\x1b[38;5;{}m", val),
            Value::Rgb { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
        }
    }
}

impl Background {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn to_ansi(&self) -> String {
        match &self.0 {
            Value::Iso { color, bright } => {
                format!("\x1b[{}{}m", if *bright { 10 } else { 4 }, color.to_char())
            }
            Value::Extended(val) => format!("\x1b[48;5;{}m", val),
            Value::Rgb { r, g, b } => format!("\x1b[48;2;{};{};{}m", r, g, b),
        }
    }
}
