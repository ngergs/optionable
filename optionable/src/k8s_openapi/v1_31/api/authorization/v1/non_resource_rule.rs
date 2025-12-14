#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NonResourceRuleAc {
    #[serde(rename = "nonResourceURLs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_urls: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbs: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authorization::v1::NonResourceRule {
    type Optioned = NonResourceRuleAc;
}
#[automatically_derived]
impl crate::Optionable for NonResourceRuleAc {
    type Optioned = NonResourceRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::NonResourceRule {
    fn into_optioned(self) -> NonResourceRuleAc {
        NonResourceRuleAc {
            non_resource_urls: crate::OptionableConvert::into_optioned(
                self.non_resource_urls,
            ),
            verbs: Some(crate::OptionableConvert::into_optioned(self.verbs)),
        }
    }
    fn try_from_optioned(value: NonResourceRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            non_resource_urls: crate::OptionableConvert::try_from_optioned(
                value.non_resource_urls,
            )?,
            verbs: crate::OptionableConvert::try_from_optioned(
                value
                    .verbs
                    .ok_or(crate::Error {
                        missing_field: "verbs",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NonResourceRuleAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.non_resource_urls,
            other.non_resource_urls,
        )?;
        if let Some(other_value) = other.verbs {
            crate::OptionableConvert::merge(&mut self.verbs, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::authorization::v1::NonResourceRule>
for NonResourceRuleAc {
    fn from_optionable(
        value: ::k8s_openapi::api::authorization::v1::NonResourceRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::authorization::v1::NonResourceRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::authorization::v1::NonResourceRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
