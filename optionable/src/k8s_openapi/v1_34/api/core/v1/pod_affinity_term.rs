#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PodAffinityTermAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_label_keys: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mismatch_label_keys: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topology_key: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::PodAffinityTerm {
    type Optioned = PodAffinityTermAc;
}
#[automatically_derived]
impl crate::Optionable for PodAffinityTermAc {
    type Optioned = PodAffinityTermAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::PodAffinityTerm {
    fn into_optioned(self) -> PodAffinityTermAc {
        PodAffinityTermAc {
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
    fn try_from_optioned(value: PodAffinityTermAc) -> Result<Self, crate::Error> {
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
                    .ok_or(crate::Error {
                        missing_field: "topology_key",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: PodAffinityTermAc) -> Result<(), crate::Error> {
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
