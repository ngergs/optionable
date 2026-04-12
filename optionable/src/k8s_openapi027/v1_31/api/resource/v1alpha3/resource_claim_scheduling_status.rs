#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceClaimSchedulingStatus contains information about one particular ResourceClaim with "WaitForFirstConsumer" allocation mode.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimSchedulingStatusAc {
    /// Name matches the pod.spec.resourceClaims\[*\].Name field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// UnsuitableNodes lists nodes that the ResourceClaim cannot be allocated for.
    ///
    /// The size of this field is limited to 128, the same as for PodSchedulingSpec.PotentialNodes. This may get increased in the future, but not reduced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuitable_nodes: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1alpha3::ResourceClaimSchedulingStatus {
    type Optioned = ResourceClaimSchedulingStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimSchedulingStatusAc {
    type Optioned = ResourceClaimSchedulingStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::ResourceClaimSchedulingStatus {
    fn into_optioned(self) -> ResourceClaimSchedulingStatusAc {
        ResourceClaimSchedulingStatusAc {
            name: Some(self.name),
            unsuitable_nodes: self.unsuitable_nodes,
        }
    }
    fn try_from_optioned(
        value: ResourceClaimSchedulingStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            unsuitable_nodes: value.unsuitable_nodes,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimSchedulingStatusAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.unsuitable_nodes.is_none() {
            self.unsuitable_nodes = crate::OptionableConvert::try_from_optioned(
                other.unsuitable_nodes,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.unsuitable_nodes,
                other.unsuitable_nodes,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1alpha3::ResourceClaimSchedulingStatus,
> for ResourceClaimSchedulingStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::ResourceClaimSchedulingStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::ResourceClaimSchedulingStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::ResourceClaimSchedulingStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
