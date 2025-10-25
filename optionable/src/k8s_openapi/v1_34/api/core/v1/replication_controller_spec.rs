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
impl crate::Optionable
for ::k8s_openapi::api::core::v1::replication_controller_spec::ReplicationControllerSpec {
    type Optioned = ReplicationControllerSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ReplicationControllerSpecOpt {
    type Optioned = ReplicationControllerSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::replication_controller_spec::ReplicationControllerSpec {
    fn into_optioned(self) -> ReplicationControllerSpecOpt {
        ReplicationControllerSpecOpt {
            min_ready_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.min_ready_seconds),
            replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.replicas),
            selector: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.selector),
            template: <Option<
                ::k8s_openapi::api::core::v1::PodTemplateSpec,
            > as crate::OptionableConvert>::into_optioned(self.template),
        }
    }
    fn try_from_optioned(
        value: ReplicationControllerSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            min_ready_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.min_ready_seconds)?,
            replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.replicas)?,
            selector: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.selector)?,
            template: <Option<
                ::k8s_openapi::api::core::v1::PodTemplateSpec,
            > as crate::OptionableConvert>::try_from_optioned(value.template)?,
        })
    }
    fn merge(
        &mut self,
        other: ReplicationControllerSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.min_ready_seconds,
            other.min_ready_seconds,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.replicas, other.replicas)?;
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.selector, other.selector)?;
        <Option<
            ::k8s_openapi::api::core::v1::PodTemplateSpec,
        > as crate::OptionableConvert>::merge(&mut self.template, other.template)?;
        Ok(())
    }
}
