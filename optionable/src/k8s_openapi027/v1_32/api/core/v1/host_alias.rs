#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostAliasAc {
    /// Hostnames for the above IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<std::vec::Vec<std::string::String>>,
    /// IP address of the host file entry.
    pub ip: std::string::String,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::HostAlias {
    type Optioned = HostAliasAc;
}
#[automatically_derived]
impl crate::Optionable for HostAliasAc {
    type Optioned = HostAliasAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::HostAlias {
    fn into_optioned(self) -> HostAliasAc {
        HostAliasAc {
            hostnames: self.hostnames,
            ip: self.ip,
        }
    }
    fn try_from_optioned(value: HostAliasAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hostnames: value.hostnames,
            ip: value.ip,
        })
    }
    fn merge(&mut self, other: HostAliasAc) -> Result<(), crate::Error> {
        if self.hostnames.is_none() {
            self.hostnames = crate::OptionableConvert::try_from_optioned(
                other.hostnames,
            )?;
        } else {
            self.hostnames = crate::OptionableConvert::try_from_optioned(
                other.hostnames,
            )?;
        }
        self.ip = other.ip;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq for k8s_openapi027::api::core::v1::HostAlias {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.ip == other.ip
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::HostAlias> for HostAliasAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::HostAlias) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::HostAlias, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::HostAlias,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
