#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitiesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: <Option<std::vec::Vec<std::string::String>> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Capabilities {
    type Optioned = CapabilitiesAc;
}
#[automatically_derived]
impl crate::Optionable for CapabilitiesAc {
    type Optioned = CapabilitiesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Capabilities {
    fn into_optioned(self) -> CapabilitiesAc {
        CapabilitiesAc {
            add: crate::OptionableConvert::into_optioned(self.add),
            drop: crate::OptionableConvert::into_optioned(self.drop),
        }
    }
    fn try_from_optioned(value: CapabilitiesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            add: crate::OptionableConvert::try_from_optioned(value.add)?,
            drop: crate::OptionableConvert::try_from_optioned(value.drop)?,
        })
    }
    fn merge(&mut self, other: CapabilitiesAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.add, other.add)?;
        crate::OptionableConvert::merge(&mut self.drop, other.drop)?;
        Ok(())
    }
}
