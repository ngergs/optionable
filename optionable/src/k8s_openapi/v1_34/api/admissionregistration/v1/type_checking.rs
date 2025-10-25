pub struct TypeCheckingOpt {
    pub expression_warnings: <Option<
        std::vec::Vec<::k8s_openapi::api::admissionregistration::v1::ExpressionWarning>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::admissionregistration::v1::TypeChecking {
    type Optioned = TypeCheckingOpt;
}
#[automatically_derived]
impl crate::Optionable for TypeCheckingOpt {
    type Optioned = TypeCheckingOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1::TypeChecking {
    fn into_optioned(self) -> TypeCheckingOpt {
        TypeCheckingOpt {
            expression_warnings: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1::ExpressionWarning,
                >,
            > as crate::OptionableConvert>::into_optioned(self.expression_warnings),
        }
    }
    fn try_from_optioned(
        value: TypeCheckingOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression_warnings: <Option<
                std::vec::Vec<
                    ::k8s_openapi::api::admissionregistration::v1::ExpressionWarning,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.expression_warnings)?,
        })
    }
    fn merge(&mut self, other: TypeCheckingOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<
                ::k8s_openapi::api::admissionregistration::v1::ExpressionWarning,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.expression_warnings,
            other.expression_warnings,
        )?;
        Ok(())
    }
}
