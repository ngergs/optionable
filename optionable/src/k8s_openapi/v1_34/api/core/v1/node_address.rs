pub struct NodeAddressOpt {
    pub address: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeAddress {
    type Optioned = NodeAddressOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeAddressOpt {
    type Optioned = NodeAddressOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeAddress {
    fn into_optioned(self) -> NodeAddressOpt {
        NodeAddressOpt {
            address: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.address,
                ),
            ),
            type_: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.type_,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: NodeAddressOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            address: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .address
                    .ok_or(crate::optionable::Error {
                        missing_field: "address",
                    })?,
            )?,
            type_: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::optionable::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NodeAddressOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.address {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.address,
                other_value,
            )?;
        }
        if let Some(other_value) = other.type_ {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.type_,
                other_value,
            )?;
        }
        Ok(())
    }
}
