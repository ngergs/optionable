#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PersistentVolumeClaimVolumeSource references the user's PVC in the same namespace. This volume finds the bound PV and mounts that volume for the pod. A PersistentVolumeClaimVolumeSource is, essentially, a wrapper around another type of volume that is owned by someone else (the system).
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PersistentVolumeClaimVolumeSourceAc {
    /// claimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_name: Option<std::string::String>,
    /// readOnly Will force the ReadOnly setting in VolumeMounts. Default false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource {
    type Optioned = PersistentVolumeClaimVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for PersistentVolumeClaimVolumeSourceAc {
    type Optioned = PersistentVolumeClaimVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource {
    fn into_optioned(self) -> PersistentVolumeClaimVolumeSourceAc {
        PersistentVolumeClaimVolumeSourceAc {
            claim_name: Some(self.claim_name),
            read_only: self.read_only,
        }
    }
    fn try_from_optioned(
        value: PersistentVolumeClaimVolumeSourceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            claim_name: value
                .claim_name
                .ok_or(crate::Error {
                    missing_field: "claim_name",
                })?,
            read_only: value.read_only,
        })
    }
    fn merge(
        &mut self,
        other: PersistentVolumeClaimVolumeSourceAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.claim_name {
            self.claim_name = other_value;
        }
        if other.read_only.is_some() {
            self.read_only = other.read_only;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource,
> for PersistentVolumeClaimVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PersistentVolumeClaimVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
