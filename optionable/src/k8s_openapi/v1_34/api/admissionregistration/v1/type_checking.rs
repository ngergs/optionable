pub struct TypeCheckingAc {
    pub expression_warnings: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::ExpressionWarning>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::admissionregistration::v1::TypeChecking {
    type Optioned = TypeCheckingAc;
}
#[automatically_derived]
impl crate::Optionable for TypeCheckingAc {
    type Optioned = TypeCheckingAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::TypeChecking {
    fn into_optioned(self) -> TypeCheckingAc {
        TypeCheckingAc {
            expression_warnings: crate::OptionableConvert::into_optioned(
                self.expression_warnings,
            ),
        }
    }
    fn try_from_optioned(
        value: TypeCheckingAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression_warnings: crate::OptionableConvert::try_from_optioned(
                value.expression_warnings,
            )?,
        })
    }
    fn merge(&mut self, other: TypeCheckingAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.expression_warnings,
            other.expression_warnings,
        )?;
        Ok(())
    }
}
