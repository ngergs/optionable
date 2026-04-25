#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// TypeChecking contains results of type checking the expressions in the ValidatingAdmissionPolicy
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TypeCheckingAc {
    /// The type checking warnings for each expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_warnings: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::admissionregistration::v1alpha1::ExpressionWarning as crate::Optionable>::Optioned,
        >,
    >,
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
        if self.expression_warnings.is_none() {
            self.expression_warnings = crate::OptionableConvert::try_from_optioned(
                other.expression_warnings,
            )?;
        } else if let Some(self_value) = self.expression_warnings.as_mut()
            && let Some(other_value) = other.expression_warnings
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
impl k8s_openapi027::DeepMerge for TypeCheckingAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.expression_warnings,
            other.expression_warnings,
        );
    }
}
