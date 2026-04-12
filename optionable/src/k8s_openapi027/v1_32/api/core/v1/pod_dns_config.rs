#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PodDNSConfig defines the DNS parameters of a pod in addition to those generated from DNSPolicy.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodDNSConfigAc {
    /// A list of DNS name server IP addresses. This will be appended to the base nameservers generated from DNSPolicy. Duplicated nameservers will be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<std::vec::Vec<std::string::String>>,
    /// A list of DNS resolver options. This will be merged with the base options generated from DNSPolicy. Duplicated entries will be removed. Resolution options given in Options will override those that appear in the base DNSPolicy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::PodDNSConfigOption as crate::Optionable>::Optioned,
        >,
    >,
    /// A list of DNS search domains for host-name lookup. This will be appended to the base search paths generated from DNSPolicy. Duplicated search paths will be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searches: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::PodDNSConfig {
    type Optioned = PodDNSConfigAc;
}
#[automatically_derived]
impl crate::Optionable for PodDNSConfigAc {
    type Optioned = PodDNSConfigAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::PodDNSConfig {
    fn into_optioned(self) -> PodDNSConfigAc {
        PodDNSConfigAc {
            nameservers: self.nameservers,
            options: crate::OptionableConvert::into_optioned(self.options),
            searches: self.searches,
        }
    }
    fn try_from_optioned(value: PodDNSConfigAc) -> Result<Self, crate::Error> {
        Ok(Self {
            nameservers: value.nameservers,
            options: crate::OptionableConvert::try_from_optioned(value.options)?,
            searches: value.searches,
        })
    }
    fn merge(&mut self, other: PodDNSConfigAc) -> Result<(), crate::Error> {
        if self.nameservers.is_none() {
            self.nameservers = other.nameservers;
        }
        if let Some(other_value) = other.nameservers {
            self.nameservers = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.options.is_none() {
            self.options = other.options;
        }
        if let Some(other_value) = other.options {
            self.options = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.searches.is_none() {
            self.searches = other.searches;
        }
        if let Some(other_value) = other.searches {
            self.searches = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::PodDNSConfig>
for PodDNSConfigAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::PodDNSConfig) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::PodDNSConfig, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::PodDNSConfig,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
