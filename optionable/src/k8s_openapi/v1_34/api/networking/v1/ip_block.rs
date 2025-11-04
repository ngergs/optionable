#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct IPBlockAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub except: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IPBlock {
    type Optioned = IPBlockAc;
}
#[automatically_derived]
impl crate::Optionable for IPBlockAc {
    type Optioned = IPBlockAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IPBlock {
    fn into_optioned(self) -> IPBlockAc {
        IPBlockAc {
            cidr: Some(crate::OptionableConvert::into_optioned(self.cidr)),
            except: crate::OptionableConvert::into_optioned(self.except),
        }
    }
    fn try_from_optioned(value: IPBlockAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            cidr: crate::OptionableConvert::try_from_optioned(
                value
                    .cidr
                    .ok_or(crate::optionable::Error {
                        missing_field: "cidr",
                    })?,
            )?,
            except: crate::OptionableConvert::try_from_optioned(value.except)?,
        })
    }
    fn merge(&mut self, other: IPBlockAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.cidr {
            crate::OptionableConvert::merge(&mut self.cidr, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.except, other.except)?;
        Ok(())
    }
}
