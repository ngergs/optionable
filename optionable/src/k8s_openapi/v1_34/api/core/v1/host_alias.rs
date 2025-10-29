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
            hostnames: crate::OptionableConvert::into_optioned(self.hostnames),
            ip: Some(crate::OptionableConvert::into_optioned(self.ip)),
        }
    }
    fn try_from_optioned(value: HostAliasOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hostnames: crate::OptionableConvert::try_from_optioned(value.hostnames)?,
            ip: crate::OptionableConvert::try_from_optioned(
                value
                    .ip
                    .ok_or(crate::optionable::Error {
                        missing_field: "ip",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: HostAliasOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.hostnames, other.hostnames)?;
        if let Some(other_value) = other.ip {
            crate::OptionableConvert::merge(&mut self.ip, other_value)?;
        }
        Ok(())
    }
}
