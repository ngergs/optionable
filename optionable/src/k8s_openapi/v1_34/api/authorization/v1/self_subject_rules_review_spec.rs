#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectRulesReviewSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::SelfSubjectRulesReviewSpec {
    type Optioned = SelfSubjectRulesReviewSpecAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectRulesReviewSpecAc {
    type Optioned = SelfSubjectRulesReviewSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SelfSubjectRulesReviewSpec {
    fn into_optioned(self) -> SelfSubjectRulesReviewSpecAc {
        SelfSubjectRulesReviewSpecAc {
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectRulesReviewSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
        })
    }
    fn merge(
        &mut self,
        other: SelfSubjectRulesReviewSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        Ok(())
    }
}
