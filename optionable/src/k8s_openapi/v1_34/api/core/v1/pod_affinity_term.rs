pub struct PodAffinityTermOpt {
    pub label_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub match_label_keys: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub mismatch_label_keys: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub namespace_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    pub namespaces: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub topology_key: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodAffinityTerm {
    type Optioned = PodAffinityTermOpt;
}
#[automatically_derived]
impl crate::Optionable for PodAffinityTermOpt {
    type Optioned = PodAffinityTermOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodAffinityTerm {
    fn into_optioned(self) -> PodAffinityTermOpt {
        PodAffinityTermOpt {
            label_selector: crate::OptionableConvert::into_optioned(self.label_selector),
            match_label_keys: crate::OptionableConvert::into_optioned(
                self.match_label_keys,
            ),
            mismatch_label_keys: crate::OptionableConvert::into_optioned(
                self.mismatch_label_keys,
            ),
            namespace_selector: crate::OptionableConvert::into_optioned(
                self.namespace_selector,
            ),
            namespaces: crate::OptionableConvert::into_optioned(self.namespaces),
            topology_key: Some(
                crate::OptionableConvert::into_optioned(self.topology_key),
            ),
        }
    }
    fn try_from_optioned(
        value: PodAffinityTermOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            label_selector: crate::OptionableConvert::try_from_optioned(
                value.label_selector,
            )?,
            match_label_keys: crate::OptionableConvert::try_from_optioned(
                value.match_label_keys,
            )?,
            mismatch_label_keys: crate::OptionableConvert::try_from_optioned(
                value.mismatch_label_keys,
            )?,
            namespace_selector: crate::OptionableConvert::try_from_optioned(
                value.namespace_selector,
            )?,
            namespaces: crate::OptionableConvert::try_from_optioned(value.namespaces)?,
            topology_key: crate::OptionableConvert::try_from_optioned(
                value
                    .topology_key
                    .ok_or(crate::optionable::Error {
                        missing_field: "topology_key",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PodAffinityTermOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.label_selector, other.label_selector)?;
        crate::OptionableConvert::merge(
            &mut self.match_label_keys,
            other.match_label_keys,
        )?;
        crate::OptionableConvert::merge(
            &mut self.mismatch_label_keys,
            other.mismatch_label_keys,
        )?;
        crate::OptionableConvert::merge(
            &mut self.namespace_selector,
            other.namespace_selector,
        )?;
        crate::OptionableConvert::merge(&mut self.namespaces, other.namespaces)?;
        if let Some(other_value) = other.topology_key {
            crate::OptionableConvert::merge(&mut self.topology_key, other_value)?;
        }
        Ok(())
    }
}
