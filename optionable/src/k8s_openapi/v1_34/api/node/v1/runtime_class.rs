#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeClassAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhead: <Option<
        ::k8s_openapi::api::node::v1::Overhead,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling: <Option<
        ::k8s_openapi::api::node::v1::Scheduling,
    > as crate::Optionable>::Optioned,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    phantom: std::marker::PhantomData<RuntimeClassAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::node::v1::RuntimeClass {
    type Optioned = RuntimeClassAc;
}
#[automatically_derived]
impl crate::Optionable for RuntimeClassAc {
    type Optioned = RuntimeClassAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::node::v1::RuntimeClass {
    fn into_optioned(self) -> RuntimeClassAc {
        RuntimeClassAc {
            handler: Some(crate::OptionableConvert::into_optioned(self.handler)),
            metadata: self.metadata,
            overhead: crate::OptionableConvert::into_optioned(self.overhead),
            scheduling: crate::OptionableConvert::into_optioned(self.scheduling),
        }
    }
    fn try_from_optioned(
        value: RuntimeClassAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            handler: crate::OptionableConvert::try_from_optioned(
                value
                    .handler
                    .ok_or(crate::optionable::Error {
                        missing_field: "handler",
                    })?,
            )?,
            metadata: value.metadata,
            overhead: crate::OptionableConvert::try_from_optioned(value.overhead)?,
            scheduling: crate::OptionableConvert::try_from_optioned(value.scheduling)?,
        })
    }
    fn merge(&mut self, other: RuntimeClassAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.handler {
            crate::OptionableConvert::merge(&mut self.handler, other_value)?;
        }
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.overhead, other.overhead)?;
        crate::OptionableConvert::merge(&mut self.scheduling, other.scheduling)?;
        Ok(())
    }
}
impl k8s_openapi::Resource for RuntimeClassAc {
    const API_VERSION: &'static str = "node.k8s.io/v1";
    const GROUP: &'static str = "node.k8s.io";
    const KIND: &'static str = "RuntimeClass";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "runtimeclasses";
    type Scope = k8s_openapi::ClusterResourceScope;
}
impl k8s_openapi::Metadata for RuntimeClassAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
