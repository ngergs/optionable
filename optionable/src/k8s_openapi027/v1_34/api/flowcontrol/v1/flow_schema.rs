#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// FlowSchema defines the schema of a group of flows. Note that a flow is made up of a set of inbound API requests with similar attributes and is identified by a pair of strings: the name of the FlowSchema and a "flow distinguisher".
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowSchemaAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    /// `metadata` is the standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// `spec` is the specification of the desired behavior of a FlowSchema. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi027::api::flowcontrol::v1::FlowSchemaSpec as crate::Optionable>::Optioned,
    >,
    /// `status` is the current status of a FlowSchema. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        <::k8s_openapi027::api::flowcontrol::v1::FlowSchemaStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::flowcontrol::v1::FlowSchema {
    type Optioned = FlowSchemaAc;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaAc {
    type Optioned = FlowSchemaAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::flowcontrol::v1::FlowSchema {
    fn into_optioned(self) -> FlowSchemaAc {
        FlowSchemaAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: FlowSchemaAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: FlowSchemaAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if self.spec.is_none() {
            self.spec = other.spec;
        }
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        if self.status.is_none() {
            self.status = other.status;
        }
        if let Some(other_value) = other.status {
            crate::OptionableConvert::merge(&mut self.status, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::flowcontrol::v1::FlowSchema>
for FlowSchemaAc {
    fn from_optionable(value: k8s_openapi027::api::flowcontrol::v1::FlowSchema) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::flowcontrol::v1::FlowSchema, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::FlowSchema,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for FlowSchemaAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::flowcontrol::v1::FlowSchema as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::flowcontrol::v1::FlowSchema as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::flowcontrol::v1::FlowSchema as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::flowcontrol::v1::FlowSchema as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::flowcontrol::v1::FlowSchema as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::flowcontrol::v1::FlowSchema as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for FlowSchemaAc {
    type Ty = <k8s_openapi027::api::flowcontrol::v1::FlowSchema as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_flowschemaac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::flowcontrol::v1::FlowSchema,
    >();
}
