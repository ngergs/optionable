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
    pub admin_access: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1beta1::DeviceSelector>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::DeviceRequest {
    type Optioned = DeviceRequestAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestAc {
    type Optioned = DeviceRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta1::DeviceRequest {
    fn into_optioned(self) -> DeviceRequestAc {
        DeviceRequestAc {
            admin_access: crate::OptionableConvert::into_optioned(self.admin_access),
            allocation_mode: crate::OptionableConvert::into_optioned(
                self.allocation_mode,
            ),
            count: crate::OptionableConvert::into_optioned(self.count),
            device_class_name: Some(
                crate::OptionableConvert::into_optioned(self.device_class_name),
            ),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
        }
    }
    fn try_from_optioned(value: DeviceRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            admin_access: crate::OptionableConvert::try_from_optioned(
                value.admin_access,
            )?,
            allocation_mode: crate::OptionableConvert::try_from_optioned(
                value.allocation_mode,
            )?,
            count: crate::OptionableConvert::try_from_optioned(value.count)?,
            device_class_name: crate::OptionableConvert::try_from_optioned(
                value
                    .device_class_name
                    .ok_or(crate::Error {
                        missing_field: "device_class_name",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
        })
    }
    fn merge(&mut self, other: DeviceRequestAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.admin_access, other.admin_access)?;
        crate::OptionableConvert::merge(
            &mut self.allocation_mode,
            other.allocation_mode,
        )?;
        crate::OptionableConvert::merge(&mut self.count, other.count)?;
        if let Some(other_value) = other.device_class_name {
            crate::OptionableConvert::merge(&mut self.device_class_name, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.selectors, other.selectors)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::resource::v1beta1::DeviceRequest>
for DeviceRequestAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1beta1::DeviceRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::resource::v1beta1::DeviceRequest, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1beta1::DeviceRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
