#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// RuntimeClass defines a class of container runtime supported in the cluster. The RuntimeClass is used to determine which container runtime is used to run all containers in a pod. RuntimeClasses are manually defined by a user or cluster provisioner, and referenced in the PodSpec. The Kubelet is responsible for resolving the RuntimeClassName reference before running the pod.  For more details, see https://kubernetes.io/docs/concepts/containers/runtime-class/
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuntimeClassAc {
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
    /// handler specifies the underlying runtime and configuration that the CRI implementation will use to handle pods of this class. The possible values are specific to the node & CRI configuration.  It is assumed that all handlers are available on every node, and handlers of the same name are equivalent on every node. For example, a handler called "runc" might specify that the runc OCI runtime (using native Linux containers) will be used to run the containers in a pod. The Handler must be lowercase, conform to the DNS Label (RFC 1123) requirements, and is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<std::string::String>,
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// overhead represents the resource overhead associated with running a pod for a given RuntimeClass. For more details, see
    ///  https://kubernetes.io/docs/concepts/scheduling-eviction/pod-overhead/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overhead: Option<
        <::k8s_openapi027::api::node::v1::Overhead as crate::Optionable>::Optioned,
    >,
    /// scheduling holds the scheduling constraints to ensure that pods running with this RuntimeClass are scheduled to nodes that support it. If scheduling is nil, this RuntimeClass is assumed to be supported by all nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<
        <::k8s_openapi027::api::node::v1::Scheduling as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::node::v1::RuntimeClass {
    type Optioned = RuntimeClassAc;
}
#[automatically_derived]
impl crate::Optionable for RuntimeClassAc {
    type Optioned = RuntimeClassAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::node::v1::RuntimeClass {
    fn into_optioned(self) -> RuntimeClassAc {
        RuntimeClassAc {
            api_version: Default::default(),
            kind: Default::default(),
            handler: Some(self.handler),
            metadata: self.metadata,
            overhead: crate::OptionableConvert::into_optioned(self.overhead),
            scheduling: crate::OptionableConvert::into_optioned(self.scheduling),
        }
    }
    fn try_from_optioned(value: RuntimeClassAc) -> Result<Self, crate::Error> {
        Ok(Self {
            handler: value
                .handler
                .ok_or(crate::Error {
                    missing_field: "handler",
                })?,
            metadata: value.metadata,
            overhead: crate::OptionableConvert::try_from_optioned(value.overhead)?,
            scheduling: crate::OptionableConvert::try_from_optioned(value.scheduling)?,
        })
    }
    fn merge(&mut self, other: RuntimeClassAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.handler {
            self.handler = other_value;
        }
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.overhead, other.overhead)?;
        crate::OptionableConvert::merge(&mut self.scheduling, other.scheduling)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::node::v1::RuntimeClass>
for RuntimeClassAc {
    fn from_optionable(value: k8s_openapi027::api::node::v1::RuntimeClass) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::node::v1::RuntimeClass, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::node::v1::RuntimeClass,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for RuntimeClassAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::node::v1::RuntimeClass as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::node::v1::RuntimeClass as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::node::v1::RuntimeClass as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::node::v1::RuntimeClass as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::node::v1::RuntimeClass as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::node::v1::RuntimeClass as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for RuntimeClassAc {
    type Ty = <k8s_openapi027::api::node::v1::RuntimeClass as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_runtimeclassac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::node::v1::RuntimeClass>();
}
