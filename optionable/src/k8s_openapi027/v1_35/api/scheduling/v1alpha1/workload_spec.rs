#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadSpec defines the desired state of a Workload.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadSpecAc {
    /// ControllerRef is an optional reference to the controlling object, such as a Deployment or Job. This field is intended for use by tools like CLIs to provide a link back to the original workload definition. When set, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_ref: Option<
        <::k8s_openapi027::api::scheduling::v1alpha1::TypedLocalObjectReference as crate::Optionable>::Optioned,
    >,
    /// PodGroups is the list of pod groups that make up the Workload. The maximum number of pod groups is 8. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_groups: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::scheduling::v1alpha1::PodGroup as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::scheduling::v1alpha1::WorkloadSpec {
    type Optioned = WorkloadSpecAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadSpecAc {
    type Optioned = WorkloadSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::scheduling::v1alpha1::WorkloadSpec {
    fn into_optioned(self) -> WorkloadSpecAc {
        WorkloadSpecAc {
            controller_ref: crate::OptionableConvert::into_optioned(self.controller_ref),
            pod_groups: Some(crate::OptionableConvert::into_optioned(self.pod_groups)),
        }
    }
    fn try_from_optioned(value: WorkloadSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            controller_ref: crate::OptionableConvert::try_from_optioned(
                value.controller_ref,
            )?,
            pod_groups: crate::OptionableConvert::try_from_optioned(
                value
                    .pod_groups
                    .ok_or(crate::Error {
                        missing_field: "pod_groups",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: WorkloadSpecAc) -> Result<(), crate::Error> {
        if self.controller_ref.is_none() {
            self.controller_ref = crate::OptionableConvert::try_from_optioned(
                other.controller_ref,
            )?;
        } else if let Some(self_value) = self.controller_ref.as_mut()
            && let Some(other_value) = other.controller_ref
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.pod_groups {
            crate::OptionableConvert::merge(&mut self.pod_groups, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::scheduling::v1alpha1::WorkloadSpec>
for WorkloadSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::scheduling::v1alpha1::WorkloadSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::scheduling::v1alpha1::WorkloadSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::scheduling::v1alpha1::WorkloadSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
