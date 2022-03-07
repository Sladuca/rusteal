use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TypePrimitive {
    Void,
    UInt64,
    Byteslice,
    Halt,
}

impl Display for TypePrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TypePrimitive::Void => "<void>",
                TypePrimitive::UInt64 => "int",
                TypePrimitive::Byteslice => "bytes",
                TypePrimitive::Halt => "<halt>",
            }
        )
    }
}

impl TryFrom<&str> for TypePrimitive {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Void" => Ok(TypePrimitive::Void),
            "UInt64" => Ok(TypePrimitive::UInt64),
            "Byteslice" => Ok(TypePrimitive::Byteslice),
            "Halt" => Ok(TypePrimitive::Halt),
            _ => Err(()),
        }
    }
}
