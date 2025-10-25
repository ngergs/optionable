pub struct CustomResourceConversionOpt {
    pub strategy: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub webhook: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion {
    type Optioned = CustomResourceConversionOpt;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceConversionOpt {
    type Optioned = CustomResourceConversionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion {
    fn into_optioned(self) -> CustomResourceConversionOpt {
        CustomResourceConversionOpt {
            strategy: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.strategy,
                ),
            ),
            webhook: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion,
            > as crate::OptionableConvert>::into_optioned(self.webhook),
        }
    }
    fn try_from_optioned(
        value: CustomResourceConversionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            strategy: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .strategy
                    .ok_or(crate::optionable::Error {
                        missing_field: "strategy",
                    })?,
            )?,
            webhook: <Option<
                ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion,
            > as crate::OptionableConvert>::try_from_optioned(value.webhook)?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceConversionOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.strategy {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.strategy,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion,
        > as crate::OptionableConvert>::merge(&mut self.webhook, other.webhook)?;
        Ok(())
    }
}
