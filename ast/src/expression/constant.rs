use crate::{
    compilation_error::CompilationError,
    context::{CompilationContext, TypeContext},
    typing::{TypeEnum, TypeError, TypePrimitive},
};

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum OnComplete {
    NoOp,
    OptIn,
    CloseOut,
    ClearState,
    UpdateApplication,
    DeleteApplication,
}

impl Expression for OnComplete {
    fn resolve(&self, _: &TypeContext) -> Result<TypeEnum, TypeError> {
        Ok(TypeEnum::Simple(TypePrimitive::UInt64))
    }

    fn compile(
        &self,
        _: &CompilationContext,
        _: &mut Vec<String>,
    ) -> Result<String, CompilationError> {
        Ok(format!("int {self:?}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::{constant::OnComplete, Expression};

    #[test]
    fn test() {
        let e = OnComplete::NoOp;
        assert_eq!(e.compile_raw().unwrap(), "int NoOp");
    }
}
