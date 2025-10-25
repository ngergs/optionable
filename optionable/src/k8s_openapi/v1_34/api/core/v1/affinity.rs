pub struct AffinityOpt {
    pub node_affinity: <Option<
        ::k8s_openapi::api::core::v1::NodeAffinity,
    > as crate::Optionable>::Optioned,
    pub pod_affinity: <Option<
        ::k8s_openapi::api::core::v1::PodAffinity,
    > as crate::Optionable>::Optioned,
    pub pod_anti_affinity: <Option<
        ::k8s_openapi::api::core::v1::PodAntiAffinity,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Affinity {
    type Optioned = AffinityOpt;
}
#[automatically_derived]
impl crate::Optionable for AffinityOpt {
    type Optioned = AffinityOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Affinity {
    fn into_optioned(self) -> AffinityOpt {
        AffinityOpt {
            node_affinity: <Option<
                ::k8s_openapi::api::core::v1::NodeAffinity,
            > as crate::OptionableConvert>::into_optioned(self.node_affinity),
            pod_affinity: <Option<
                ::k8s_openapi::api::core::v1::PodAffinity,
            > as crate::OptionableConvert>::into_optioned(self.pod_affinity),
            pod_anti_affinity: <Option<
                ::k8s_openapi::api::core::v1::PodAntiAffinity,
            > as crate::OptionableConvert>::into_optioned(self.pod_anti_affinity),
        }
    }
    fn try_from_optioned(value: AffinityOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            node_affinity: <Option<
                ::k8s_openapi::api::core::v1::NodeAffinity,
            > as crate::OptionableConvert>::try_from_optioned(value.node_affinity)?,
            pod_affinity: <Option<
                ::k8s_openapi::api::core::v1::PodAffinity,
            > as crate::OptionableConvert>::try_from_optioned(value.pod_affinity)?,
            pod_anti_affinity: <Option<
                ::k8s_openapi::api::core::v1::PodAntiAffinity,
            > as crate::OptionableConvert>::try_from_optioned(value.pod_anti_affinity)?,
        })
    }
    fn merge(&mut self, other: AffinityOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::NodeAffinity,
        > as crate::OptionableConvert>::merge(
            &mut self.node_affinity,
            other.node_affinity,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::PodAffinity,
        > as crate::OptionableConvert>::merge(
            &mut self.pod_affinity,
            other.pod_affinity,
        )?;
        <Option<
            ::k8s_openapi::api::core::v1::PodAntiAffinity,
        > as crate::OptionableConvert>::merge(
            &mut self.pod_anti_affinity,
            other.pod_anti_affinity,
        )?;
        Ok(())
    }
}
