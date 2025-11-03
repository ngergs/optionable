#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindingAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<
        <::k8s_openapi::api::core::v1::ObjectReference as crate::Optionable>::Optioned,
    >,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    pub phantom: std::marker::PhantomData<BindingAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Binding {
    type Optioned = BindingAc;
}
#[automatically_derived]
impl crate::Optionable for BindingAc {
    type Optioned = BindingAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Binding {
    fn into_optioned(self) -> BindingAc {
        BindingAc {
            metadata: self.metadata,
            target: Some(crate::OptionableConvert::into_optioned(self.target)),
            phantom: Default::default(),
        }
    }
    fn try_from_optioned(value: BindingAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            target: crate::OptionableConvert::try_from_optioned(
                value
                    .target
                    .ok_or(crate::optionable::Error {
                        missing_field: "target",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: BindingAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.target {
            crate::OptionableConvert::merge(&mut self.target, other_value)?;
        }
        Ok(())
    }
}
impl k8s_openapi::Resource for BindingAc {
    const API_VERSION: &'static str = <::k8s_openapi::api::core::v1::Binding as k8s_openapi::Resource>::API_VERSION;
    const GROUP: &'static str = <::k8s_openapi::api::core::v1::Binding as k8s_openapi::Resource>::GROUP;
    const KIND: &'static str = <::k8s_openapi::api::core::v1::Binding as k8s_openapi::Resource>::KIND;
    const VERSION: &'static str = <::k8s_openapi::api::core::v1::Binding as k8s_openapi::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <::k8s_openapi::api::core::v1::Binding as k8s_openapi::Resource>::URL_PATH_SEGMENT;
    type Scope = <::k8s_openapi::api::core::v1::Binding as k8s_openapi::Resource>::Scope;
}
impl k8s_openapi::Metadata for BindingAc {
    type Ty = <::k8s_openapi::api::core::v1::Binding as k8s_openapi::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
