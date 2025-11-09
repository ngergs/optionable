#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
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
