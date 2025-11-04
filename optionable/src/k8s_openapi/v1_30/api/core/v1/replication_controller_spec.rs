#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ReplicationControllerSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: <Option<
        ::k8s_openapi::api::core::v1::PodTemplateSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ReplicationControllerSpec {
    type Optioned = ReplicationControllerSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ReplicationControllerSpecAc {
    type Optioned = ReplicationControllerSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ReplicationControllerSpec {
    fn into_optioned(self) -> ReplicationControllerSpecAc {
        ReplicationControllerSpecAc {
            min_ready_seconds: crate::OptionableConvert::into_optioned(
                self.min_ready_seconds,
            ),
            replicas: crate::OptionableConvert::into_optioned(self.replicas),
            selector: crate::OptionableConvert::into_optioned(self.selector),
            template: crate::OptionableConvert::into_optioned(self.template),
        }
    }
    fn try_from_optioned(
        value: ReplicationControllerSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            min_ready_seconds: crate::OptionableConvert::try_from_optioned(
                value.min_ready_seconds,
            )?,
            replicas: crate::OptionableConvert::try_from_optioned(value.replicas)?,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
            template: crate::OptionableConvert::try_from_optioned(value.template)?,
        })
    }
    fn merge(
        &mut self,
        other: ReplicationControllerSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.min_ready_seconds,
            other.min_ready_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.replicas, other.replicas)?;
        crate::OptionableConvert::merge(&mut self.selector, other.selector)?;
        crate::OptionableConvert::merge(&mut self.template, other.template)?;
        Ok(())
    }
}
