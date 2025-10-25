pub struct IPBlockOpt {
    pub cidr: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub except: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IPBlock {
    type Optioned = IPBlockOpt;
}
#[automatically_derived]
impl crate::Optionable for IPBlockOpt {
    type Optioned = IPBlockOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IPBlock {
    fn into_optioned(self) -> IPBlockOpt {
        IPBlockOpt {
            cidr: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.cidr,
                ),
            ),
            except: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.except),
        }
    }
    fn try_from_optioned(value: IPBlockOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            cidr: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .cidr
                    .ok_or(crate::optionable::Error {
                        missing_field: "cidr",
                    })?,
            )?,
            except: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.except)?,
        })
    }
    fn merge(&mut self, other: IPBlockOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.cidr {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.cidr,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.except, other.except)?;
        Ok(())
    }
}
