use strum_macros::EnumString;

use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typing::{TypeEnum, TypeError, TypePrimitive},
    OP_SEPARATOR,
};

use super::Expression;

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum Ret {
    Approve,
    Reject,
}

impl Expression for Ret {
    fn resolve(&self, _: &TypeContext) -> Result<TypeEnum, TypeError> {
        Ok(TypeEnum::Simple(TypePrimitive::Halt))
    }

    fn compile(
        &self,
        _: &CompilationContext,
        _: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        Ok(format!(
            "int {value}{OP_SEPARATOR}return",
            value = match self {
                Ret::Approve => "1",
                Ret::Reject => "0",
            }
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        context::TypeContext,
        expression::{ret::Ret, Expression},
    };

    #[test]
    fn test() {
        let e = Ret::Approve;
        println!("{:?}", e.resolve(&TypeContext::default()));
        println!("{}", e.compile_raw().unwrap());
    }
}
