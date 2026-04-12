#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Device represents one individual hardware instance that can be selected based on its attributes. Besides the name, exactly one field must be set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceAc {
    /// Basic defines one device instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic: Option<
        <::k8s_openapi027::api::resource::v1beta1::BasicDevice as crate::Optionable>::Optioned,
    >,
    /// Name is unique identifier among all devices managed by the driver in the pool. It must be a DNS label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta1::Device {
    type Optioned = DeviceAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceAc {
    type Optioned = DeviceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1beta1::Device {
    fn into_optioned(self) -> DeviceAc {
        DeviceAc {
            basic: crate::OptionableConvert::into_optioned(self.basic),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(value: DeviceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            basic: crate::OptionableConvert::try_from_optioned(value.basic)?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
        })
    }
    fn merge(&mut self, other: DeviceAc) -> Result<(), crate::Error> {
        if self.basic.is_none() {
            self.basic = crate::OptionableConvert::try_from_optioned(other.basic)?;
        } else {
            crate::OptionableConvert::merge(&mut self.basic, other.basic)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta1::Device>
for DeviceAc {
    fn from_optionable(value: k8s_openapi027::api::resource::v1beta1::Device) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta1::Device, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::Device,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
