#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ServiceCIDRSpec define the CIDRs the user wants to use for allocating ClusterIPs for Services.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceCIDRSpecAc {
    /// CIDRs defines the IP blocks in CIDR notation (e.g. "192.168.0.0/24" or "2001:db8::/64") from which to assign service cluster IPs. Max of two CIDRs is allowed, one of each IP family. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidrs: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1beta1::ServiceCIDRSpec {
    type Optioned = ServiceCIDRSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceCIDRSpecAc {
    type Optioned = ServiceCIDRSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1beta1::ServiceCIDRSpec {
    fn into_optioned(self) -> ServiceCIDRSpecAc {
        ServiceCIDRSpecAc {
            cidrs: self.cidrs,
        }
    }
    fn try_from_optioned(value: ServiceCIDRSpecAc) -> Result<Self, crate::Error> {
        Ok(Self { cidrs: value.cidrs })
    }
    fn merge(&mut self, other: ServiceCIDRSpecAc) -> Result<(), crate::Error> {
        if self.cidrs.is_none() {
            self.cidrs = other.cidrs;
        }
        if let Some(other_value) = other.cidrs {
            self.cidrs = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1beta1::ServiceCIDRSpec>
for ServiceCIDRSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1beta1::ServiceCIDRSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::networking::v1beta1::ServiceCIDRSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1beta1::ServiceCIDRSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
