pub struct ReplicationControllerSpecOpt {
    pub min_ready_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub selector: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub template: <Option<
        ::k8s_openapi::api::core::v1::PodTemplateSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ReplicationControllerSpec {
    type Optioned = ReplicationControllerSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ReplicationControllerSpecOpt {
    type Optioned = ReplicationControllerSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ReplicationControllerSpec {
    fn into_optioned(self) -> ReplicationControllerSpecOpt {
        ReplicationControllerSpecOpt {
            min_ready_seconds: crate::OptionableConvert::into_optioned(
                self.min_ready_seconds,
            ),
            replicas: crate::OptionableConvert::into_optioned(self.replicas),
            selector: crate::OptionableConvert::into_optioned(self.selector),
            template: crate::OptionableConvert::into_optioned(self.template),
        }
    }
    fn try_from_optioned(
        value: ReplicationControllerSpecOpt,
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
        other: ReplicationControllerSpecOpt,
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
