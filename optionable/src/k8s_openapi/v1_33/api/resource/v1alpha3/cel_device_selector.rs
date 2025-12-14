#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CELDeviceSelectorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector {
    type Optioned = CELDeviceSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for CELDeviceSelectorAc {
    type Optioned = CELDeviceSelectorAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector {
    fn into_optioned(self) -> CELDeviceSelectorAc {
        CELDeviceSelectorAc {
            expression: Some(crate::OptionableConvert::into_optioned(self.expression)),
        }
    }
    fn try_from_optioned(value: CELDeviceSelectorAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: crate::OptionableConvert::try_from_optioned(
                value
                    .expression
                    .ok_or(crate::Error {
                        missing_field: "expression",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CELDeviceSelectorAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.expression {
            crate::OptionableConvert::merge(&mut self.expression, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector>
for CELDeviceSelectorAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
