#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConstraintAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_attribute: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta2::DeviceConstraint {
    type Optioned = DeviceConstraintAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceConstraintAc {
    type Optioned = DeviceConstraintAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::DeviceConstraint {
    fn into_optioned(self) -> DeviceConstraintAc {
        DeviceConstraintAc {
            match_attribute: crate::OptionableConvert::into_optioned(
                self.match_attribute,
            ),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(value: DeviceConstraintAc) -> Result<Self, crate::Error> {
        Ok(Self {
            match_attribute: crate::OptionableConvert::try_from_optioned(
                value.match_attribute,
            )?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: DeviceConstraintAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.match_attribute,
            other.match_attribute,
        )?;
        crate::OptionableConvert::merge(&mut self.requests, other.requests)?;
        Ok(())
    }
}
