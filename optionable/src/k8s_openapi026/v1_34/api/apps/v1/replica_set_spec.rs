#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReplicaSetSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ready_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi026::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: <Option<
        ::k8s_openapi026::api::core::v1::PodTemplateSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::apps::v1::ReplicaSetSpec {
    type Optioned = ReplicaSetSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ReplicaSetSpecAc {
    type Optioned = ReplicaSetSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::apps::v1::ReplicaSetSpec {
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
    fn try_from_optioned(value: ReplicaSetSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            min_ready_seconds: crate::OptionableConvert::try_from_optioned(
                value.min_ready_seconds,
            )?,
            replicas: crate::OptionableConvert::try_from_optioned(value.replicas)?,
            selector: crate::OptionableConvert::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::Error {
                        missing_field: "selector",
                    })?,
            )?,
            template: crate::OptionableConvert::try_from_optioned(value.template)?,
        })
    }
    fn merge(&mut self, other: ReplicaSetSpecAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::apps::v1::ReplicaSetSpec>
for ReplicaSetSpecAc {
    fn from_optionable(value: k8s_openapi026::api::apps::v1::ReplicaSetSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::apps::v1::ReplicaSetSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::apps::v1::ReplicaSetSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
