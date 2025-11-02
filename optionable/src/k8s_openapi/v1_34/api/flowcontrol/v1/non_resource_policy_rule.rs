#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonResourcePolicyRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_urls: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbs: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::NonResourcePolicyRule {
    type Optioned = NonResourcePolicyRuleAc;
}
#[automatically_derived]
impl crate::Optionable for NonResourcePolicyRuleAc {
    type Optioned = NonResourcePolicyRuleAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::NonResourcePolicyRule {
    fn into_optioned(self) -> NonResourcePolicyRuleAc {
        NonResourcePolicyRuleAc {
            non_resource_urls: Some(
                crate::OptionableConvert::into_optioned(self.non_resource_urls),
            ),
            verbs: Some(crate::OptionableConvert::into_optioned(self.verbs)),
        }
    }
    fn try_from_optioned(
        value: NonResourcePolicyRuleAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            non_resource_urls: crate::OptionableConvert::try_from_optioned(
                value
                    .non_resource_urls
                    .ok_or(crate::optionable::Error {
                        missing_field: "non_resource_urls",
                    })?,
            )?,
            verbs: crate::OptionableConvert::try_from_optioned(
                value
                    .verbs
                    .ok_or(crate::optionable::Error {
                        missing_field: "verbs",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NonResourcePolicyRuleAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.non_resource_urls {
            crate::OptionableConvert::merge(&mut self.non_resource_urls, other_value)?;
        }
        if let Some(other_value) = other.verbs {
            crate::OptionableConvert::merge(&mut self.verbs, other_value)?;
        }
        Ok(())
    }
}
