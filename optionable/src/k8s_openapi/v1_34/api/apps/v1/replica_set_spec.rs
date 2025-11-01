pub struct ReplicaSetSpecAc {
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
impl crate::Optionable for ::k8s_openapi::api::apps::v1::ReplicaSetSpec {
    type Optioned = ReplicaSetSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ReplicaSetSpecAc {
    type Optioned = ReplicaSetSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::ReplicaSetSpec {
    fn into_optioned(self) -> ReplicaSetSpecAc {
        ReplicaSetSpecAc {
            min_ready_seconds: crate::OptionableConvert::into_optioned(
                self.min_ready_seconds,
            ),
            replicas: crate::OptionableConvert::into_optioned(self.replicas),
            selector: Some(crate::OptionableConvert::into_optioned(self.selector)),
            template: crate::OptionableConvert::into_optioned(self.template),
        }
    }
    fn try_from_optioned(
        value: ReplicaSetSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            min_ready_seconds: crate::OptionableConvert::try_from_optioned(
                value.min_ready_seconds,
            )?,
            replicas: crate::OptionableConvert::try_from_optioned(value.replicas)?,
            selector: crate::OptionableConvert::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::optionable::Error {
                        missing_field: "selector",
                    })?,
            )?,
            template: crate::OptionableConvert::try_from_optioned(value.template)?,
        })
    }
    fn merge(
        &mut self,
        other: ReplicaSetSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.min_ready_seconds,
            other.min_ready_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.replicas, other.replicas)?;
        if let Some(other_value) = other.selector {
            crate::OptionableConvert::merge(&mut self.selector, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.template, other.template)?;
        Ok(())
    }
}
