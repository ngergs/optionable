#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic: <Option<
        ::k8s_openapi::api::resource::v1alpha3::BasicDevice,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::Device {
    type Optioned = DeviceAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAc {
    type Optioned = DeviceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1alpha3::Device {
    fn into_optioned(self) -> DeviceAc {
        DeviceAc {
            basic: crate::OptionableConvert::into_optioned(self.basic),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(value: DeviceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            basic: crate::OptionableConvert::try_from_optioned(value.basic)?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: DeviceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.basic, other.basic)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
