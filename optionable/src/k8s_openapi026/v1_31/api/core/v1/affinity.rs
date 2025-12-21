#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AffinityAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_affinity: <Option<
        ::k8s_openapi026::api::core::v1::NodeAffinity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_affinity: <Option<
        ::k8s_openapi026::api::core::v1::PodAffinity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_anti_affinity: <Option<
        ::k8s_openapi026::api::core::v1::PodAntiAffinity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::Affinity {
    type Optioned = AffinityAc;
}
#[automatically_derived]
impl crate::Optionable for AffinityAc {
    type Optioned = AffinityAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::Affinity {
    fn into_optioned(self) -> AffinityAc {
        AffinityAc {
            node_affinity: crate::OptionableConvert::into_optioned(self.node_affinity),
            pod_affinity: crate::OptionableConvert::into_optioned(self.pod_affinity),
            pod_anti_affinity: crate::OptionableConvert::into_optioned(
                self.pod_anti_affinity,
            ),
        }
    }
    fn try_from_optioned(value: AffinityAc) -> Result<Self, crate::Error> {
        Ok(Self {
            node_affinity: crate::OptionableConvert::try_from_optioned(
                value.node_affinity,
            )?,
            pod_affinity: crate::OptionableConvert::try_from_optioned(
                value.pod_affinity,
            )?,
            pod_anti_affinity: crate::OptionableConvert::try_from_optioned(
                value.pod_anti_affinity,
            )?,
        })
    }
    fn merge(&mut self, other: AffinityAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.node_affinity, other.node_affinity)?;
        crate::OptionableConvert::merge(&mut self.pod_affinity, other.pod_affinity)?;
        crate::OptionableConvert::merge(
            &mut self.pod_anti_affinity,
            other.pod_anti_affinity,
        )?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::Affinity> for AffinityAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::Affinity) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::Affinity, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::Affinity,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
