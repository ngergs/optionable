#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceRequestAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_access: Option<bool>,
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
            <::k8s_openapi027::api::resource::v1alpha3::DeviceSelector as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::DeviceRequest {
    type Optioned = DeviceRequestAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestAc {
    type Optioned = DeviceRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::DeviceRequest {
    fn into_optioned(self) -> DeviceRequestAc {
        DeviceRequestAc {
            admin_access: self.admin_access,
            allocation_mode: self.allocation_mode,
            count: self.count,
            device_class_name: Some(self.device_class_name),
            name: Some(self.name),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
        }
    }
    fn try_from_optioned(value: DeviceRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            admin_access: value.admin_access,
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
        })
    }
    fn merge(&mut self, other: DeviceRequestAc) -> Result<(), crate::Error> {
        self.admin_access = other.admin_access;
        self.allocation_mode = other.allocation_mode;
        self.count = other.count;
        if let Some(other_value) = other.device_class_name {
            self.device_class_name = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        crate::OptionableConvert::merge(&mut self.selectors, other.selectors)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::DeviceRequest>
for DeviceRequestAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1alpha3::DeviceRequest, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
