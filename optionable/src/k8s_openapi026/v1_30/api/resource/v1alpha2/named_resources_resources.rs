#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamedResourcesResourcesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<
        <std::vec::Vec<
            ::k8s_openapi026::api::resource::v1alpha2::NamedResourcesInstance,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::resource::v1alpha2::NamedResourcesResources {
    type Optioned = NamedResourcesResourcesAc;
}
#[automatically_derived]
impl crate::Optionable for NamedResourcesResourcesAc {
    type Optioned = NamedResourcesResourcesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::resource::v1alpha2::NamedResourcesResources {
    fn into_optioned(self) -> NamedResourcesResourcesAc {
        NamedResourcesResourcesAc {
            instances: Some(crate::OptionableConvert::into_optioned(self.instances)),
        }
    }
    fn try_from_optioned(
        value: NamedResourcesResourcesAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            instances: crate::OptionableConvert::try_from_optioned(
                value
                    .instances
                    .ok_or(crate::Error {
                        missing_field: "instances",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NamedResourcesResourcesAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.instances {
            crate::OptionableConvert::merge(&mut self.instances, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::resource::v1alpha2::NamedResourcesResources,
> for NamedResourcesResourcesAc {
    fn from_optionable(
        value: k8s_openapi026::api::resource::v1alpha2::NamedResourcesResources,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::resource::v1alpha2::NamedResourcesResources,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::resource::v1alpha2::NamedResourcesResources,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
