pub struct NonResourceRuleOpt {
    pub non_resource_urls: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub verbs: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::non_resource_rule::NonResourceRule {
    type Optioned = NonResourceRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for NonResourceRuleOpt {
    type Optioned = NonResourceRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::non_resource_rule::NonResourceRule {
    fn into_optioned(self) -> NonResourceRuleOpt {
        NonResourceRuleOpt {
            non_resource_urls: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.non_resource_urls),
            verbs: Some(
                <std::vec::Vec<
                    std::string::String,
                > as crate::OptionableConvert>::into_optioned(self.verbs),
            ),
        }
    }
    fn try_from_optioned(
        value: NonResourceRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            non_resource_urls: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.non_resource_urls)?,
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
        other: NonResourceRuleOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(
            &mut self.non_resource_urls,
            other.non_resource_urls,
        )?;
        if let Some(other_value) = other.verbs {
            <std::vec::Vec<
                std::string::String,
            > as crate::OptionableConvert>::merge(&mut self.verbs, other_value)?;
        }
        Ok(())
    }
}
