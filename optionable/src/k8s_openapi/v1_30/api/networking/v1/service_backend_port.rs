#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ServiceBackendPortAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::ServiceBackendPort {
    type Optioned = ServiceBackendPortAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceBackendPortAc {
    type Optioned = ServiceBackendPortAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::ServiceBackendPort {
    fn into_optioned(self) -> ServiceBackendPortAc {
        ServiceBackendPortAc {
            name: crate::OptionableConvert::into_optioned(self.name),
            number: crate::OptionableConvert::into_optioned(self.number),
        }
    }
    fn try_from_optioned(value: ServiceBackendPortAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            number: crate::OptionableConvert::try_from_optioned(value.number)?,
        })
    }
    fn merge(&mut self, other: ServiceBackendPortAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.number, other.number)?;
        Ok(())
    }
}
