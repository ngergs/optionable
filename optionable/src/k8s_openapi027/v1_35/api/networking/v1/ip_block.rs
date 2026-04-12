#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IPBlock describes a particular CIDR (Ex. "192.168.1.0/24","2001:db8::/64") that is allowed to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs that should not be included within this rule.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IPBlockAc {
    /// cidr is a string representing the IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<std::string::String>,
    /// except is a slice of CIDRs that should not be included within an IPBlock Valid examples are "192.168.1.0/24" or "2001:db8::/64" Except values will be rejected if they are outside the cidr range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub except: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IPBlock {
    type Optioned = IPBlockAc;
}
#[automatically_derived]
impl crate::Optionable for IPBlockAc {
    type Optioned = IPBlockAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::IPBlock {
    fn into_optioned(self) -> IPBlockAc {
        IPBlockAc {
            cidr: Some(self.cidr),
            except: self.except,
        }
    }
    fn try_from_optioned(value: IPBlockAc) -> Result<Self, crate::Error> {
        Ok(Self {
            cidr: value
                .cidr
                .ok_or(crate::Error {
                    missing_field: "cidr",
                })?,
            except: value.except,
        })
    }
    fn merge(&mut self, other: IPBlockAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.cidr {
            self.cidr = other_value;
        }
        if other.except.is_some() {
            self.except = other.except;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IPBlock> for IPBlockAc {
    fn from_optionable(value: k8s_openapi027::api::networking::v1::IPBlock) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::IPBlock, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IPBlock,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
