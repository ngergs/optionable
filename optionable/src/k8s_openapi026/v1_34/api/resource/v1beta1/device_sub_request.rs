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
    pub allocation_mode: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: <Option<
        ::k8s_openapi026::api::resource::v1beta1::CapacityRequirements,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: <Option<
        std::vec::Vec<::k8s_openapi026::api::resource::v1beta1::DeviceSelector>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerations: <Option<
        std::vec::Vec<::k8s_openapi026::api::resource::v1beta1::DeviceToleration>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::resource::v1beta1::DeviceSubRequest {
    type Optioned = DeviceSubRequestAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceSubRequestAc {
    type Optioned = DeviceSubRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::resource::v1beta1::DeviceSubRequest {
    fn into_optioned(self) -> DeviceSubRequestAc {
        DeviceSubRequestAc {
            allocation_mode: crate::OptionableConvert::into_optioned(
                self.allocation_mode,
            ),
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
            count: crate::OptionableConvert::into_optioned(self.count),
            device_class_name: Some(
                crate::OptionableConvert::into_optioned(self.device_class_name),
            ),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(value: DeviceSubRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            allocation_mode: crate::OptionableConvert::try_from_optioned(
                value.allocation_mode,
            )?,
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
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
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(&mut self, other: DeviceSubRequestAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.allocation_mode,
            other.allocation_mode,
        )?;
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        crate::OptionableConvert::merge(&mut self.count, other.count)?;
        if let Some(other_value) = other.device_class_name {
            crate::OptionableConvert::merge(&mut self.device_class_name, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.selectors, other.selectors)?;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::resource::v1beta1::DeviceSubRequest>
for DeviceSubRequestAc {
    fn from_optionable(
        value: k8s_openapi026::api::resource::v1beta1::DeviceSubRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::resource::v1beta1::DeviceSubRequest, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::resource::v1beta1::DeviceSubRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
