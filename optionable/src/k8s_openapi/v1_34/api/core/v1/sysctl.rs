pub struct SysctlOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub value: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Sysctl {
    type Optioned = SysctlOpt;
}
#[automatically_derived]
impl crate::Optionable for SysctlOpt {
    type Optioned = SysctlOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Sysctl {
    fn into_optioned(self) -> SysctlOpt {
        SysctlOpt {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            value: Some(crate::OptionableConvert::into_optioned(self.value)),
        }
    }
    fn try_from_optioned(value: SysctlOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            value: crate::OptionableConvert::try_from_optioned(
                value
                    .value
                    .ok_or(crate::optionable::Error {
                        missing_field: "value",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: SysctlOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.value {
            crate::OptionableConvert::merge(&mut self.value, other_value)?;
        }
        Ok(())
    }
}
