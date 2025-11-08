#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceConversionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::WebhookConversion,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion {
    type Optioned = CustomResourceConversionAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceConversionAc {
    type Optioned = CustomResourceConversionAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceConversion {
    fn into_optioned(self) -> CustomResourceConversionAc {
        CustomResourceConversionAc {
            strategy: Some(crate::OptionableConvert::into_optioned(self.strategy)),
            webhook: crate::OptionableConvert::into_optioned(self.webhook),
        }
    }
    fn try_from_optioned(
        value: CustomResourceConversionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            strategy: crate::OptionableConvert::try_from_optioned(
                value
                    .strategy
                    .ok_or(crate::Error {
                        missing_field: "strategy",
                    })?,
            )?,
            webhook: crate::OptionableConvert::try_from_optioned(value.webhook)?,
        })
    }
    fn merge(&mut self, other: CustomResourceConversionAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.strategy {
            crate::OptionableConvert::merge(&mut self.strategy, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.webhook, other.webhook)?;
        Ok(())
    }
}
