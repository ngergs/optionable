#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PodSchedulingContextStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claims: <Option<
        std::vec::Vec<
            ::k8s_openapi::api::resource::v1alpha3::ResourceClaimSchedulingStatus,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha3::PodSchedulingContextStatus {
    type Optioned = PodSchedulingContextStatusAc;
}
#[automatically_derived]
impl crate::Optionable for PodSchedulingContextStatusAc {
    type Optioned = PodSchedulingContextStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::PodSchedulingContextStatus {
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
        crate::OptionableConvert::merge(
            &mut self.resource_claims,
            other.resource_claims,
        )?;
        Ok(())
    }
}
