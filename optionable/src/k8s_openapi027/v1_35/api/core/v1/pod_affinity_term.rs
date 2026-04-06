#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key \<topologyKey\> matches that of any node on which a pod of the set of pods is running
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodAffinityTermAc {
    /// A label query over a set of resources, in this case pods. If it's null, this PodAffinityTerm matches with no Pods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// MatchLabelKeys is a set of pod label keys to select which pods will be taken into consideration. The keys are used to lookup values from the incoming pod labels, those key-value labels are merged with `labelSelector` as `key in (value)` to select the group of existing pods which pods will be taken into consideration for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming pod labels will be ignored. The default value is empty. The same key is forbidden to exist in both matchLabelKeys and labelSelector. Also, matchLabelKeys cannot be set when labelSelector isn't set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_label_keys: Option<std::vec::Vec<std::string::String>>,
    /// MismatchLabelKeys is a set of pod label keys to select which pods will be taken into consideration. The keys are used to lookup values from the incoming pod labels, those key-value labels are merged with `labelSelector` as `key notin (value)` to select the group of existing pods which pods will be taken into consideration for the incoming pod's pod (anti) affinity. Keys that don't exist in the incoming pod labels will be ignored. The default value is empty. The same key is forbidden to exist in both mismatchLabelKeys and labelSelector. Also, mismatchLabelKeys cannot be set when labelSelector isn't set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mismatch_label_keys: Option<std::vec::Vec<std::string::String>>,
    /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<std::vec::Vec<std::string::String>>,
    /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_key: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodAffinityTerm {
    type Optioned = PodAffinityTermAc;
}
#[automatically_derived]
impl crate::Optionable for PodAffinityTermAc {
    type Optioned = PodAffinityTermAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodAffinityTerm {
    fn into_optioned(self) -> PodAffinityTermAc {
        PodAffinityTermAc {
            label_selector: crate::OptionableConvert::into_optioned(self.label_selector),
            match_label_keys: self.match_label_keys,
            mismatch_label_keys: self.mismatch_label_keys,
            namespace_selector: crate::OptionableConvert::into_optioned(
                self.namespace_selector,
            ),
            namespaces: self.namespaces,
            topology_key: Some(self.topology_key),
        }
    }
    fn try_from_optioned(value: PodAffinityTermAc) -> Result<Self, crate::Error> {
        Ok(Self {
            label_selector: crate::OptionableConvert::try_from_optioned(
                value.label_selector,
            )?,
            match_label_keys: value.match_label_keys,
            mismatch_label_keys: value.mismatch_label_keys,
            namespace_selector: crate::OptionableConvert::try_from_optioned(
                value.namespace_selector,
            )?,
            namespaces: value.namespaces,
            topology_key: value
                .topology_key
                .ok_or(crate::Error {
                    missing_field: "topology_key",
                })?,
        })
    }
    fn merge(&mut self, other: PodAffinityTermAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.label_selector, other.label_selector)?;
        self.match_label_keys = other.match_label_keys;
        self.mismatch_label_keys = other.mismatch_label_keys;
        crate::OptionableConvert::merge(
            &mut self.namespace_selector,
            other.namespace_selector,
        )?;
        self.namespaces = other.namespaces;
        if let Some(other_value) = other.topology_key {
            self.topology_key = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodAffinityTerm>
for PodAffinityTermAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodAffinityTerm) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodAffinityTerm, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodAffinityTerm,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
