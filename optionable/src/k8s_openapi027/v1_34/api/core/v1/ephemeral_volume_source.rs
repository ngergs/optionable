#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Represents an ephemeral volume that is handled by a normal storage driver.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EphemeralVolumeSourceAc {
    /// Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `\<pod name\>-\<volume name\>` where `\<volume name\>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long).
    ///
    /// An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster.
    ///
    /// This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created.
    ///
    /// Required, must not be nil.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_claim_template: Option<
        <::k8s_openapi027::api::core::v1::PersistentVolumeClaimTemplate as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EphemeralVolumeSource {
    type Optioned = EphemeralVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EphemeralVolumeSourceAc {
    type Optioned = EphemeralVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EphemeralVolumeSource {
    fn into_optioned(self) -> EphemeralVolumeSourceAc {
        EphemeralVolumeSourceAc {
            volume_claim_template: crate::OptionableConvert::into_optioned(
                self.volume_claim_template,
            ),
        }
    }
    fn try_from_optioned(value: EphemeralVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            volume_claim_template: crate::OptionableConvert::try_from_optioned(
                value.volume_claim_template,
            )?,
        })
    }
    fn merge(&mut self, other: EphemeralVolumeSourceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.volume_claim_template,
            other.volume_claim_template,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EphemeralVolumeSource>
for EphemeralVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::EphemeralVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EphemeralVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EphemeralVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
