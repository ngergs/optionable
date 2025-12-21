#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimSchedulingStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuitable_nodes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::resource::v1alpha2::ResourceClaimSchedulingStatus {
    type Optioned = ResourceClaimSchedulingStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimSchedulingStatusAc {
    type Optioned = ResourceClaimSchedulingStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::resource::v1alpha2::ResourceClaimSchedulingStatus {
    fn into_optioned(self) -> ResourceClaimSchedulingStatusAc {
        ResourceClaimSchedulingStatusAc {
            name: crate::OptionableConvert::into_optioned(self.name),
            unsuitable_nodes: crate::OptionableConvert::into_optioned(
                self.unsuitable_nodes,
            ),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimSchedulingStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            unsuitable_nodes: crate::OptionableConvert::try_from_optioned(
                value.unsuitable_nodes,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimSchedulingStatusAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(
            &mut self.unsuitable_nodes,
            other.unsuitable_nodes,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::resource::v1alpha2::ResourceClaimSchedulingStatus,
> for ResourceClaimSchedulingStatusAc {
    fn from_optionable(
        value: k8s_openapi026::api::resource::v1alpha2::ResourceClaimSchedulingStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::resource::v1alpha2::ResourceClaimSchedulingStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::resource::v1alpha2::ResourceClaimSchedulingStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
