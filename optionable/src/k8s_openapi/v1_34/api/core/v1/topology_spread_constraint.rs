pub struct TopologySpreadConstraintOpt {
    pub label_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub match_label_keys: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub max_skew: Option<i32>,
    pub min_domains: <Option<i32> as crate::Optionable>::Optioned,
    pub node_affinity_policy: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub node_taints_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub topology_key: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub when_unsatisfiable: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::TopologySpreadConstraint {
    type Optioned = TopologySpreadConstraintOpt;
}
#[automatically_derived]
impl crate::Optionable for TopologySpreadConstraintOpt {
    type Optioned = TopologySpreadConstraintOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::TopologySpreadConstraint {
    fn into_optioned(self) -> TopologySpreadConstraintOpt {
        TopologySpreadConstraintOpt {
            label_selector: crate::OptionableConvert::into_optioned(self.label_selector),
            match_label_keys: crate::OptionableConvert::into_optioned(
                self.match_label_keys,
            ),
            max_skew: Some(self.max_skew),
            min_domains: crate::OptionableConvert::into_optioned(self.min_domains),
            node_affinity_policy: crate::OptionableConvert::into_optioned(
                self.node_affinity_policy,
            ),
            node_taints_policy: crate::OptionableConvert::into_optioned(
                self.node_taints_policy,
            ),
            topology_key: Some(
                crate::OptionableConvert::into_optioned(self.topology_key),
            ),
            when_unsatisfiable: Some(
                crate::OptionableConvert::into_optioned(self.when_unsatisfiable),
            ),
        }
    }
    fn try_from_optioned(
        value: TopologySpreadConstraintOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            label_selector: crate::OptionableConvert::try_from_optioned(
                value.label_selector,
            )?,
            match_label_keys: crate::OptionableConvert::try_from_optioned(
                value.match_label_keys,
            )?,
            max_skew: value
                .max_skew
                .ok_or(crate::optionable::Error {
                    missing_field: "max_skew",
                })?,
            min_domains: crate::OptionableConvert::try_from_optioned(value.min_domains)?,
            node_affinity_policy: crate::OptionableConvert::try_from_optioned(
                value.node_affinity_policy,
            )?,
            node_taints_policy: crate::OptionableConvert::try_from_optioned(
                value.node_taints_policy,
            )?,
            topology_key: crate::OptionableConvert::try_from_optioned(
                value
                    .topology_key
                    .ok_or(crate::optionable::Error {
                        missing_field: "topology_key",
                    })?,
            )?,
            when_unsatisfiable: crate::OptionableConvert::try_from_optioned(
                value
                    .when_unsatisfiable
                    .ok_or(crate::optionable::Error {
                        missing_field: "when_unsatisfiable",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: TopologySpreadConstraintOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.label_selector, other.label_selector)?;
        crate::OptionableConvert::merge(
            &mut self.match_label_keys,
            other.match_label_keys,
        )?;
        if let Some(other_value) = other.max_skew {
            self.max_skew = other_value;
        }
        crate::OptionableConvert::merge(&mut self.min_domains, other.min_domains)?;
        crate::OptionableConvert::merge(
            &mut self.node_affinity_policy,
            other.node_affinity_policy,
        )?;
        crate::OptionableConvert::merge(
            &mut self.node_taints_policy,
            other.node_taints_policy,
        )?;
        if let Some(other_value) = other.topology_key {
            crate::OptionableConvert::merge(&mut self.topology_key, other_value)?;
        }
        if let Some(other_value) = other.when_unsatisfiable {
            crate::OptionableConvert::merge(&mut self.when_unsatisfiable, other_value)?;
        }
        Ok(())
    }
}
