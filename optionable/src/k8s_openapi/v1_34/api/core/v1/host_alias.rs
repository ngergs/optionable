pub struct HostAliasOpt {
    pub hostnames: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub ip: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::HostAlias {
    type Optioned = HostAliasOpt;
}
#[automatically_derived]
impl crate::Optionable for HostAliasOpt {
    type Optioned = HostAliasOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::HostAlias {
    fn into_optioned(self) -> HostAliasOpt {
        HostAliasOpt {
            hostnames: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.hostnames),
            ip: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(self.ip),
            ),
        }
    }
    fn try_from_optioned(value: HostAliasOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hostnames: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.hostnames)?,
            ip: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .ip
                    .ok_or(crate::optionable::Error {
                        missing_field: "ip",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: HostAliasOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.hostnames, other.hostnames)?;
        if let Some(other_value) = other.ip {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.ip,
                other_value,
            )?;
        }
        Ok(())
    }
}
