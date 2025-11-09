#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CustomResourceValidationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_api_v3_schema: <Option<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation {
    type Optioned = CustomResourceValidationAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceValidationAc {
    type Optioned = CustomResourceValidationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation {
    fn into_optioned(self) -> CustomResourceValidationAc {
        CustomResourceValidationAc {
            open_api_v3_schema: crate::OptionableConvert::into_optioned(
                self.open_api_v3_schema,
            ),
        }
    }
    fn try_from_optioned(
        value: CustomResourceValidationAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            open_api_v3_schema: crate::OptionableConvert::try_from_optioned(
                value.open_api_v3_schema,
            )?,
        })
    }
    fn merge(&mut self, other: CustomResourceValidationAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.open_api_v3_schema,
            other.open_api_v3_schema,
        )?;
        Ok(())
    }
}
