pub struct RoleRefOpt {
    pub api_group: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::RoleRef {
    type Optioned = RoleRefOpt;
}
#[automatically_derived]
impl crate::Optionable for RoleRefOpt {
    type Optioned = RoleRefOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::RoleRef {
    fn into_optioned(self) -> RoleRefOpt {
        RoleRefOpt {
            api_group: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.api_group,
                ),
            ),
            kind: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.kind,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
        }
    }
    fn try_from_optioned(value: RoleRefOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_group: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .api_group
                    .ok_or(crate::optionable::Error {
                        missing_field: "api_group",
                    })?,
            )?,
            kind: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::optionable::Error {
                        missing_field: "kind",
                    })?,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: RoleRefOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.api_group {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.api_group,
                other_value,
            )?;
        }
        if let Some(other_value) = other.kind {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.kind,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        Ok(())
    }
}
