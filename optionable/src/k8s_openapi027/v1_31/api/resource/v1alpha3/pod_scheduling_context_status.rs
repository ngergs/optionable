#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodSchedulingContextStatus describes where resources for the Pod can be allocated.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodSchedulingContextStatusAc {
    /// ResourceClaims describes resource availability for each pod.spec.resourceClaim entry where the corresponding ResourceClaim uses "WaitForFirstConsumer" allocation mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claims: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1alpha3::ResourceClaimSchedulingStatus as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextStatus {
    type Optioned = PodSchedulingContextStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodSchedulingContextStatusAc {
    type Optioned = PodSchedulingContextStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextStatus {
    fn into_optioned(self) -> PodSchedulingContextStatusAc {
        PodSchedulingContextStatusAc {
            resource_claims: crate::OptionableConvert::into_optioned(
                self.resource_claims,
            ),
        }
    }
    fn try_from_optioned(
        value: PodSchedulingContextStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            resource_claims: crate::OptionableConvert::try_from_optioned(
                value.resource_claims,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodSchedulingContextStatusAc,
    ) -> Result<(), crate::Error> {
        if self.resource_claims.is_none() {
            self.resource_claims = crate::OptionableConvert::try_from_optioned(
                other.resource_claims,
            )?;
        } else if let Some(self_value) = self.resource_claims.as_mut()
            && let Some(other_value) = other.resource_claims
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextStatus,
> for PodSchedulingContextStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::PodSchedulingContextStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
