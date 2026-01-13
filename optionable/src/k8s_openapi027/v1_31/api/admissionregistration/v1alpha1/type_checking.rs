#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypeCheckingAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_warnings: <Option<
        std::vec::Vec<
            ::k8s_openapi027::api::admissionregistration::v1alpha1::ExpressionWarning,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::TypeChecking {
    type Optioned = TypeCheckingAc;
}
#[automatically_derived]
impl crate::Optionable for TypeCheckingAc {
    type Optioned = TypeCheckingAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::TypeChecking {
    fn into_optioned(self) -> TypeCheckingAc {
        TypeCheckingAc {
            expression_warnings: crate::OptionableConvert::into_optioned(
                self.expression_warnings,
            ),
        }
    }
    fn try_from_optioned(value: TypeCheckingAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression_warnings: crate::OptionableConvert::try_from_optioned(
                value.expression_warnings,
            )?,
        })
    }
    fn merge(&mut self, other: TypeCheckingAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.expression_warnings,
            other.expression_warnings,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::TypeChecking,
> for TypeCheckingAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::TypeChecking,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::TypeChecking,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::TypeChecking,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
