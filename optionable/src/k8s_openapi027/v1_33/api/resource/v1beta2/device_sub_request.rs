#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceSubRequestAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_mode: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta2::DeviceSelector as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta2::DeviceToleration as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta2::DeviceSubRequest {
    type Optioned = DeviceSubRequestAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceSubRequestAc {
    type Optioned = DeviceSubRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta2::DeviceSubRequest {
    fn into_optioned(self) -> DeviceSubRequestAc {
        DeviceSubRequestAc {
            allocation_mode: self.allocation_mode,
            count: self.count,
            device_class_name: Some(self.device_class_name),
            name: Some(self.name),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(value: DeviceSubRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocation_mode: value.allocation_mode,
            count: value.count,
            device_class_name: value
                .device_class_name
                .ok_or(crate::Error {
                    missing_field: "device_class_name",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(&mut self, other: DeviceSubRequestAc) -> Result<(), crate::Error> {
        self.allocation_mode = other.allocation_mode;
        self.count = other.count;
        if let Some(other_value) = other.device_class_name {
            self.device_class_name = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        crate::OptionableConvert::merge(&mut self.selectors, other.selectors)?;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta2::DeviceSubRequest>
for DeviceSubRequestAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::DeviceSubRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta2::DeviceSubRequest, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::DeviceSubRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
