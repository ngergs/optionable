#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceHandleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_data: <Option<
        ::k8s_openapi::api::resource::v1alpha2::StructuredResourceHandle,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha2::ResourceHandle {
    type Optioned = ResourceHandleAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceHandleAc {
    type Optioned = ResourceHandleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::ResourceHandle {
    fn into_optioned(self) -> ResourceHandleAc {
        ResourceHandleAc {
            data: crate::OptionableConvert::into_optioned(self.data),
            driver_name: crate::OptionableConvert::into_optioned(self.driver_name),
            structured_data: crate::OptionableConvert::into_optioned(
                self.structured_data,
            ),
        }
    }
    fn try_from_optioned(value: ResourceHandleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            data: crate::OptionableConvert::try_from_optioned(value.data)?,
            driver_name: crate::OptionableConvert::try_from_optioned(value.driver_name)?,
            structured_data: crate::OptionableConvert::try_from_optioned(
                value.structured_data,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceHandleAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.data, other.data)?;
        crate::OptionableConvert::merge(&mut self.driver_name, other.driver_name)?;
        crate::OptionableConvert::merge(
            &mut self.structured_data,
            other.structured_data,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::resource::v1alpha2::ResourceHandle>
for ResourceHandleAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha2::ResourceHandle,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::resource::v1alpha2::ResourceHandle, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha2::ResourceHandle,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
