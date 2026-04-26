#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CSIDriver captures information about a Container Storage Interface (CSI) volume driver deployed on the cluster. Kubernetes attach detach controller uses this object to determine whether attach is required. Kubelet uses this object to determine whether pod information needs to be passed on mount. CSIDriver objects are non-namespaced.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSIDriverAc {
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
    /// Standard object metadata. metadata.Name indicates the name of the CSI driver that this object refers to; it MUST be the same name returned by the CSI GetPluginName() call for that driver. The driver name must be 63 characters or less, beginning and ending with an alphanumeric character (\[a-z0-9A-Z\]) with dashes (-), dots (.), and alphanumerics between. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// spec represents the specification of the CSI Driver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi027::api::storage::v1::CSIDriverSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::CSIDriver {
    type Optioned = CSIDriverAc;
}
#[automatically_derived]
impl crate::Optionable for CSIDriverAc {
    type Optioned = CSIDriverAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::storage::v1::CSIDriver {
    fn into_optioned(self) -> CSIDriverAc {
        CSIDriverAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(value: CSIDriverAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CSIDriverAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::CSIDriver>
for CSIDriverAc {
    fn from_optionable(value: k8s_openapi027::api::storage::v1::CSIDriver) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::CSIDriver, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::CSIDriver,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for CSIDriverAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::storage::v1::CSIDriver as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::storage::v1::CSIDriver as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::storage::v1::CSIDriver as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::storage::v1::CSIDriver as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::storage::v1::CSIDriver as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::storage::v1::CSIDriver as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for CSIDriverAc {
    type Ty = <k8s_openapi027::api::storage::v1::CSIDriver as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_csidriverac() {
    crate::testutil::roundtrip_test::<k8s_openapi027::api::storage::v1::CSIDriver>();
}
impl k8s_openapi027::DeepMerge for CSIDriverAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi027::DeepMerge::merge_from(&mut self.spec, other.spec);
    }
}
