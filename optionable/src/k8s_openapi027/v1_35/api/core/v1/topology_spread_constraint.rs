#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TopologySpreadConstraintAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_label_keys: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_skew: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_domains: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_affinity_policy: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_taints_policy: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_key: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_unsatisfiable: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::TopologySpreadConstraint {
    type Optioned = TopologySpreadConstraintAc;
}
#[automatically_derived]
impl crate::Optionable for TopologySpreadConstraintAc {
    type Optioned = TopologySpreadConstraintAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::TopologySpreadConstraint {
    fn into_optioned(self) -> TopologySpreadConstraintAc {
        TopologySpreadConstraintAc {
            label_selector: crate::OptionableConvert::into_optioned(self.label_selector),
            match_label_keys: self.match_label_keys,
            max_skew: Some(self.max_skew),
            min_domains: self.min_domains,
            node_affinity_policy: self.node_affinity_policy,
            node_taints_policy: self.node_taints_policy,
            topology_key: Some(self.topology_key),
            when_unsatisfiable: Some(self.when_unsatisfiable),
        }
    }
    fn try_from_optioned(
        value: TopologySpreadConstraintAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            label_selector: crate::OptionableConvert::try_from_optioned(
                value.label_selector,
            )?,
            match_label_keys: value.match_label_keys,
            max_skew: value
                .max_skew
                .ok_or(crate::Error {
                    missing_field: "max_skew",
                })?,
            min_domains: value.min_domains,
            node_affinity_policy: value.node_affinity_policy,
            node_taints_policy: value.node_taints_policy,
            topology_key: value
                .topology_key
                .ok_or(crate::Error {
                    missing_field: "topology_key",
                })?,
            when_unsatisfiable: value
                .when_unsatisfiable
                .ok_or(crate::Error {
                    missing_field: "when_unsatisfiable",
                })?,
        })
    }
    fn merge(&mut self, other: TopologySpreadConstraintAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.label_selector, other.label_selector)?;
        self.match_label_keys = other.match_label_keys;
        if let Some(other_value) = other.max_skew {
            self.max_skew = other_value;
        }
        self.min_domains = other.min_domains;
        self.node_affinity_policy = other.node_affinity_policy;
        self.node_taints_policy = other.node_taints_policy;
        if let Some(other_value) = other.topology_key {
            self.topology_key = other_value;
        }
        if let Some(other_value) = other.when_unsatisfiable {
            self.when_unsatisfiable = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::TopologySpreadConstraint>
for TopologySpreadConstraintAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::TopologySpreadConstraint,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::TopologySpreadConstraint, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::TopologySpreadConstraint,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
