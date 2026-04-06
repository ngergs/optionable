#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IPBlockAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<std::string::String>,
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
        self.except = other.except;
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
