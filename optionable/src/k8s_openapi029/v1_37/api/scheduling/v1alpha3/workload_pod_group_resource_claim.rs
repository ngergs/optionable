#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WorkloadPodGroupResourceClaim references a dynamic resource claim that is shared across pods in the group.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkloadPodGroupResourceClaimAc {
    /// name uniquely identifies this resource claim inside the group. This field is required. It must be a DNS_LABEL.
    pub name: std::string::String,
    /// resourceClaimName is the name of a ResourceClaim object in the same namespace. This field is optional. If it is not specified, no resource claim is used. If set, it must be a DNS subdomain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_name: Option<std::string::String>,
    /// resourceClaimTemplateName is the name of a ResourceClaimTemplate object in the same namespace. This field is optional. If it is not specified, no resource claim template is used. If set, it must be a DNS subdomain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_template_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim {
    type Optioned = WorkloadPodGroupResourceClaimAc;
}
#[automatically_derived]
impl crate::Optionable for WorkloadPodGroupResourceClaimAc {
    type Optioned = WorkloadPodGroupResourceClaimAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim {
    fn into_optioned(self) -> WorkloadPodGroupResourceClaimAc {
        WorkloadPodGroupResourceClaimAc {
            name: self.name,
            resource_claim_name: self.resource_claim_name,
            resource_claim_template_name: self.resource_claim_template_name,
        }
    }
    fn try_from_optioned(
        value: WorkloadPodGroupResourceClaimAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            resource_claim_name: value.resource_claim_name,
            resource_claim_template_name: value.resource_claim_template_name,
        })
    }
    fn merge(
        &mut self,
        other: WorkloadPodGroupResourceClaimAc,
    ) -> Result<(), crate::Error> {
        self.name = other.name;
        if self.resource_claim_name.is_none() {
            self.resource_claim_name = crate::OptionableConvert::try_from_optioned(
                other.resource_claim_name,
            )?;
        } else if let Some(self_value) = self.resource_claim_name.as_mut()
            && let Some(other_value) = other.resource_claim_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.resource_claim_template_name.is_none() {
            self.resource_claim_template_name = crate::OptionableConvert::try_from_optioned(
                other.resource_claim_template_name,
            )?;
        } else if let Some(self_value) = self.resource_claim_template_name.as_mut()
            && let Some(other_value) = other.resource_claim_template_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.name == other.name
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim,
> for WorkloadPodGroupResourceClaimAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1alpha3::WorkloadPodGroupResourceClaim,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for WorkloadPodGroupResourceClaimAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.resource_claim_name,
            other.resource_claim_name,
        );
        k8s_openapi029::DeepMerge::merge_from(
            &mut self.resource_claim_template_name,
            other.resource_claim_template_name,
        );
    }
}
impl crate::merge::MapKeysEq for WorkloadPodGroupResourceClaimAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
