#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfSubjectRulesReviewSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::authorization::v1::SelfSubjectRulesReviewSpec {
    type Optioned = SelfSubjectRulesReviewSpecAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectRulesReviewSpecAc {
    type Optioned = SelfSubjectRulesReviewSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::authorization::v1::SelfSubjectRulesReviewSpec {
    fn into_optioned(self) -> SelfSubjectRulesReviewSpecAc {
        SelfSubjectRulesReviewSpecAc {
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectRulesReviewSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
        })
    }
    fn merge(
        &mut self,
        other: SelfSubjectRulesReviewSpecAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::authorization::v1::SelfSubjectRulesReviewSpec,
> for SelfSubjectRulesReviewSpecAc {
    fn from_optionable(
        value: k8s_openapi026::api::authorization::v1::SelfSubjectRulesReviewSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::authorization::v1::SelfSubjectRulesReviewSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::authorization::v1::SelfSubjectRulesReviewSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
