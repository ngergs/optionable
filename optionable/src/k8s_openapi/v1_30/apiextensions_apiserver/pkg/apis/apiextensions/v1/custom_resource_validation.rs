#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceValidationAc {
    #[serde(rename = "openAPIV3Schema")]
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation,
> for CustomResourceValidationAc {
    fn from_optionable(
        value: ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceValidation,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
