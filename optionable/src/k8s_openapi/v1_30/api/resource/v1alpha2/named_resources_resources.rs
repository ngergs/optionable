#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NamedResourcesResourcesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::resource::v1alpha2::NamedResourcesInstance,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesResources {
    type Optioned = NamedResourcesResourcesAc;
}
#[automatically_derived]
impl crate::Optionable for NamedResourcesResourcesAc {
    type Optioned = NamedResourcesResourcesAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesResources {
    fn into_optioned(self) -> NamedResourcesResourcesAc {
        NamedResourcesResourcesAc {
            instances: Some(crate::OptionableConvert::into_optioned(self.instances)),
        }
    }
    fn try_from_optioned(
        value: NamedResourcesResourcesAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            instances: crate::OptionableConvert::try_from_optioned(
                value
                    .instances
                    .ok_or(crate::optionable::Error {
                        missing_field: "instances",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NamedResourcesResourcesAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.instances {
            crate::OptionableConvert::merge(&mut self.instances, other_value)?;
        }
        Ok(())
    }
}
