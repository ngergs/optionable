#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeleteOptions may be provided when deleting an API object.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeleteOptionsAc {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<std::string::String>,
    /// When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<std::vec::Vec<std::string::String>>,
    /// The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace_period_seconds: Option<i64>,
    /// if set to true, it will trigger an unsafe deletion of the resource in case the normal deletion flow fails with a corrupt object error. A resource is considered corrupt if it can not be retrieved from the underlying storage successfully because of a) its data can not be transformed e.g. decryption failure, or b) it fails to decode into an object. NOTE: unsafe deletion ignores finalizer constraints, skips precondition checks, and removes the object from the storage. WARNING: This may potentially break the cluster if the workload associated with the resource being unsafe-deleted relies on normal deletion flow. Use only if you REALLY know what you are doing. The default value is false, and the user must opt in to enable it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_store_read_error_with_cluster_breaking_potential: Option<bool>,
    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orphan_dependents: Option<bool>,
    /// Must be fulfilled before a deletion is carried out. If not possible, a 409 Conflict status will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preconditions: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Preconditions as crate::Optionable>::Optioned,
    >,
    /// Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagation_policy: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::DeleteOptions {
    type Optioned = DeleteOptionsAc;
}
#[automatically_derived]
impl crate::Optionable for DeleteOptionsAc {
    type Optioned = DeleteOptionsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::DeleteOptions {
    fn into_optioned(self) -> DeleteOptionsAc {
        DeleteOptionsAc {
            api_version: self.api_version,
            dry_run: self.dry_run,
            grace_period_seconds: self.grace_period_seconds,
            ignore_store_read_error_with_cluster_breaking_potential: self
                .ignore_store_read_error_with_cluster_breaking_potential,
            kind: self.kind,
            orphan_dependents: self.orphan_dependents,
            preconditions: crate::OptionableConvert::into_optioned(self.preconditions),
            propagation_policy: self.propagation_policy,
        }
    }
    fn try_from_optioned(value: DeleteOptionsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: value.api_version,
            dry_run: value.dry_run,
            grace_period_seconds: value.grace_period_seconds,
            ignore_store_read_error_with_cluster_breaking_potential: value
                .ignore_store_read_error_with_cluster_breaking_potential,
            kind: value.kind,
            orphan_dependents: value.orphan_dependents,
            preconditions: crate::OptionableConvert::try_from_optioned(
                value.preconditions,
            )?,
            propagation_policy: value.propagation_policy,
        })
    }
    fn merge(&mut self, other: DeleteOptionsAc) -> Result<(), crate::Error> {
        if self.api_version.is_none() {
            self.api_version = crate::OptionableConvert::try_from_optioned(
                other.api_version,
            )?;
        } else if let Some(self_value) = self.api_version.as_mut()
            && let Some(other_value) = other.api_version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.dry_run.is_none() {
            self.dry_run = crate::OptionableConvert::try_from_optioned(other.dry_run)?;
        } else if let Some(self_value) = self.dry_run.as_mut()
            && let Some(other_value) = other.dry_run
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.grace_period_seconds.is_none() {
            self.grace_period_seconds = crate::OptionableConvert::try_from_optioned(
                other.grace_period_seconds,
            )?;
        } else if let Some(self_value) = self.grace_period_seconds.as_mut()
            && let Some(other_value) = other.grace_period_seconds
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.ignore_store_read_error_with_cluster_breaking_potential.is_none() {
            self.ignore_store_read_error_with_cluster_breaking_potential = crate::OptionableConvert::try_from_optioned(
                other.ignore_store_read_error_with_cluster_breaking_potential,
            )?;
        } else if let Some(self_value) = self
            .ignore_store_read_error_with_cluster_breaking_potential
            .as_mut()
            && let Some(other_value) = other
                .ignore_store_read_error_with_cluster_breaking_potential
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.kind.is_none() {
            self.kind = crate::OptionableConvert::try_from_optioned(other.kind)?;
        } else if let Some(self_value) = self.kind.as_mut()
            && let Some(other_value) = other.kind
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.orphan_dependents.is_none() {
            self.orphan_dependents = crate::OptionableConvert::try_from_optioned(
                other.orphan_dependents,
            )?;
        } else if let Some(self_value) = self.orphan_dependents.as_mut()
            && let Some(other_value) = other.orphan_dependents
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.preconditions.is_none() {
            self.preconditions = crate::OptionableConvert::try_from_optioned(
                other.preconditions,
            )?;
        } else if let Some(self_value) = self.preconditions.as_mut()
            && let Some(other_value) = other.preconditions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.propagation_policy.is_none() {
            self.propagation_policy = crate::OptionableConvert::try_from_optioned(
                other.propagation_policy,
            )?;
        } else if let Some(self_value) = self.propagation_policy.as_mut()
            && let Some(other_value) = other.propagation_policy
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::DeleteOptions,
> for DeleteOptionsAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::DeleteOptions,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::DeleteOptions,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::DeleteOptions,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for DeleteOptionsAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        self.dry_run = other.dry_run;
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.grace_period_seconds,
            other.grace_period_seconds,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.ignore_store_read_error_with_cluster_breaking_potential,
            other.ignore_store_read_error_with_cluster_breaking_potential,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.kind, other.kind);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.orphan_dependents,
            other.orphan_dependents,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.preconditions,
            other.preconditions,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.propagation_policy,
            other.propagation_policy,
        );
    }
}
