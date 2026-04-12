#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IPAddress represents a single IP of a single IP Family. The object is designed to be used by APIs that operate on IP addresses. The object is used by the Service core API for allocation of IP addresses. An IP address can be represented in different formats, to guarantee the uniqueness of the IP, the name of the object is the IP address in canonical format, four decimal digits separated by dots suppressing leading zeros for IPv4 and the representation defined by RFC 5952 for IPv6. Valid: 192.168.1.5 or 2001:db8::1 or 2001:db8:aaaa:bbbb:cccc:dddd:eeee:1 Invalid: 10.01.2.3 or 2001:db8:0:0:0::1
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IPAddressAc {
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
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// spec is the desired state of the IPAddress. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi027::api::networking::v1beta1::IPAddressSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1beta1::IPAddress {
    type Optioned = IPAddressAc;
}
#[automatically_derived]
impl crate::Optionable for IPAddressAc {
    type Optioned = IPAddressAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1beta1::IPAddress {
    fn into_optioned(self) -> IPAddressAc {
        IPAddressAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(value: IPAddressAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: IPAddressAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if self.spec.is_none() {
            self.spec = crate::OptionableConvert::try_from_optioned(other.spec)?;
        } else if let Some(self_value) = self.spec.as_mut()
            && let Some(other_value) = other.spec
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1beta1::IPAddress>
for IPAddressAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1beta1::IPAddress,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1beta1::IPAddress, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1beta1::IPAddress,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for IPAddressAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::networking::v1beta1::IPAddress as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::networking::v1beta1::IPAddress as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::networking::v1beta1::IPAddress as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::networking::v1beta1::IPAddress as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::networking::v1beta1::IPAddress as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::networking::v1beta1::IPAddress as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for IPAddressAc {
    type Ty = <k8s_openapi027::api::networking::v1beta1::IPAddress as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_ipaddressac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::networking::v1beta1::IPAddress,
    >();
}
