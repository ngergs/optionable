#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_group: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_group_replica_key: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::WorkloadReference {
    type Optioned = WorkloadReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadReferenceAc {
    type Optioned = WorkloadReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::WorkloadReference {
    fn into_optioned(self) -> WorkloadReferenceAc {
        WorkloadReferenceAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            pod_group: Some(crate::OptionableConvert::into_optioned(self.pod_group)),
            pod_group_replica_key: crate::OptionableConvert::into_optioned(
                self.pod_group_replica_key,
            ),
        }
    }
    fn try_from_optioned(value: WorkloadReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            pod_group: crate::OptionableConvert::try_from_optioned(
                value
                    .pod_group
                    .ok_or(crate::Error {
                        missing_field: "pod_group",
                    })?,
            )?,
            pod_group_replica_key: crate::OptionableConvert::try_from_optioned(
                value.pod_group_replica_key,
            )?,
        })
    }
    fn merge(&mut self, other: WorkloadReferenceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.pod_group {
            crate::OptionableConvert::merge(&mut self.pod_group, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.pod_group_replica_key,
            other.pod_group_replica_key,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::WorkloadReference>
for WorkloadReferenceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::WorkloadReference) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::WorkloadReference, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::WorkloadReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
