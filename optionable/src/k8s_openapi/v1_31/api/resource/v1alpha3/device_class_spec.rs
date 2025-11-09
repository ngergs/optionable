#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1alpha3::DeviceClassConfiguration>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectors: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1alpha3::DeviceSelector>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitable_nodes: <Option<
        ::k8s_openapi::api::core::v1::NodeSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::DeviceClassSpec {
    type Optioned = DeviceClassSpecAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClassSpecAc {
    type Optioned = DeviceClassSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::DeviceClassSpec {
    fn into_optioned(self) -> DeviceClassSpecAc {
        DeviceClassSpecAc {
            config: crate::OptionableConvert::into_optioned(self.config),
            selectors: crate::OptionableConvert::into_optioned(self.selectors),
            suitable_nodes: crate::OptionableConvert::into_optioned(self.suitable_nodes),
        }
    }
    fn try_from_optioned(value: DeviceClassSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            selectors: crate::OptionableConvert::try_from_optioned(value.selectors)?,
            suitable_nodes: crate::OptionableConvert::try_from_optioned(
                value.suitable_nodes,
            )?,
        })
    }
    fn merge(&mut self, other: DeviceClassSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.config, other.config)?;
        crate::OptionableConvert::merge(&mut self.selectors, other.selectors)?;
        crate::OptionableConvert::merge(&mut self.suitable_nodes, other.suitable_nodes)?;
        Ok(())
    }
}
