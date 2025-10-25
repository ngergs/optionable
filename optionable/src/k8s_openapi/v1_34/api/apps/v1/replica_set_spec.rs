pub struct ReplicaSetSpecOpt {
    pub min_ready_seconds: <Option<i32> as crate::Optionable>::Optioned,
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
    pub selector: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    pub template: <Option<
        ::k8s_openapi::api::core::v1::PodTemplateSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::apps::v1::replica_set_spec::ReplicaSetSpec {
    type Optioned = ReplicaSetSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ReplicaSetSpecOpt {
    type Optioned = ReplicaSetSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::apps::v1::replica_set_spec::ReplicaSetSpec {
    fn into_optioned(self) -> ReplicaSetSpecOpt {
        ReplicaSetSpecOpt {
            min_ready_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.min_ready_seconds),
            replicas: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.replicas),
            selector: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::OptionableConvert>::into_optioned(
                    self.selector,
                ),
            ),
            template: <Option<
                ::k8s_openapi::api::core::v1::PodTemplateSpec,
            > as crate::OptionableConvert>::into_optioned(self.template),
        }
    }
    fn try_from_optioned(
        value: ReplicaSetSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            min_ready_seconds: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.min_ready_seconds)?,
            replicas: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.replicas)?,
            selector: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::OptionableConvert>::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::optionable::Error {
                        missing_field: "selector",
                    })?,
            )?,
            template: <Option<
                ::k8s_openapi::api::core::v1::PodTemplateSpec,
            > as crate::OptionableConvert>::try_from_optioned(value.template)?,
        })
    }
    fn merge(
        &mut self,
        other: ReplicaSetSpecOpt,
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
        if let Some(other_value) = other.selector {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::OptionableConvert>::merge(
                &mut self.selector,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::core::v1::PodTemplateSpec,
        > as crate::OptionableConvert>::merge(&mut self.template, other.template)?;
        Ok(())
    }
}
