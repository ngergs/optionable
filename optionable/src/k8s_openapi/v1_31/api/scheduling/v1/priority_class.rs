#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct PriorityClassAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_default: <Option<bool> as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemption_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<PriorityClassAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::scheduling::v1::PriorityClass {
    type Optioned = PriorityClassAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityClassAc {
    type Optioned = PriorityClassAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::scheduling::v1::PriorityClass {
    fn into_optioned(self) -> PriorityClassAc {
        PriorityClassAc {
            description: crate::OptionableConvert::into_optioned(self.description),
            global_default: crate::OptionableConvert::into_optioned(self.global_default),
            metadata: self.metadata,
            preemption_policy: crate::OptionableConvert::into_optioned(
                self.preemption_policy,
            ),
            value: Some(self.value),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: PriorityClassAc) -> Result<Self, crate::Error> {
        Ok(Self {
            description: crate::OptionableConvert::try_from_optioned(value.description)?,
            global_default: crate::OptionableConvert::try_from_optioned(
                value.global_default,
            )?,
            metadata: value.metadata,
            preemption_policy: crate::OptionableConvert::try_from_optioned(
                value.preemption_policy,
            )?,
            value: value
                .value
                .ok_or(crate::Error {
                    missing_field: "value",
                })?,
        })
    }
    fn merge(&mut self, other: PriorityClassAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.description, other.description)?;
        crate::OptionableConvert::merge(&mut self.global_default, other.global_default)?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(
            &mut self.preemption_policy,
            other.preemption_policy,
        )?;
        if let Some(other_value) = other.value {
            self.value = other_value;
        }
        Ok(())
    }
}
impl k8s_openapi::Resource for PriorityClassAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::scheduling::v1::PriorityClass as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::scheduling::v1::PriorityClass as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::scheduling::v1::PriorityClass as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::scheduling::v1::PriorityClass as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::scheduling::v1::PriorityClass as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::scheduling::v1::PriorityClass as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for PriorityClassAc {
    type Ty = <::k8s_openapi::api::scheduling::v1::PriorityClass as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
