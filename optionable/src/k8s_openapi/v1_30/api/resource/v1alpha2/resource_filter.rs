#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceFilterAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_resources: <Option<
        ::k8s_openapi::api::resource::v1alpha2::NamedResourcesFilter,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha2::ResourceFilter {
    type Optioned = ResourceFilterAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceFilterAc {
    type Optioned = ResourceFilterAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::ResourceFilter {
    fn into_optioned(self) -> ResourceFilterAc {
        ResourceFilterAc {
            driver_name: crate::OptionableConvert::into_optioned(self.driver_name),
            named_resources: crate::OptionableConvert::into_optioned(
                self.named_resources,
            ),
        }
    }
    fn try_from_optioned(
        value: ResourceFilterAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            driver_name: crate::OptionableConvert::try_from_optioned(value.driver_name)?,
            named_resources: crate::OptionableConvert::try_from_optioned(
                value.named_resources,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceFilterAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.driver_name, other.driver_name)?;
        crate::OptionableConvert::merge(
            &mut self.named_resources,
            other.named_resources,
        )?;
        Ok(())
    }
}
