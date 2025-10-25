pub struct NonResourcePolicyRuleOpt {
    pub non_resource_urls: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
    pub verbs: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::NonResourcePolicyRule {
    type Optioned = NonResourcePolicyRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for NonResourcePolicyRuleOpt {
    type Optioned = NonResourcePolicyRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::NonResourcePolicyRule {
    fn into_optioned(self) -> NonResourcePolicyRuleOpt {
        NonResourcePolicyRuleOpt {
            non_resource_urls: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(self.non_resource_urls),
            ),
            verbs: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(self.verbs),
            ),
        }
    }
    fn try_from_optioned(
        value: NonResourcePolicyRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            non_resource_urls: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .non_resource_urls
                    .ok_or(crate::optionable::Error {
                        missing_field: "non_resource_urls",
                    })?,
            )?,
            verbs: <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
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
        other: NonResourcePolicyRuleOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.non_resource_urls {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(
                &mut self.non_resource_urls,
                other_value,
            )?;
        }
        if let Some(other_value) = other.verbs {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(&mut self.verbs, other_value)?;
        }
        Ok(())
    }
}
