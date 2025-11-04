#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct RoleRefAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_group: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::RoleRef {
    type Optioned = RoleRefAc;
}
#[automatically_derived]
impl crate::Optionable for RoleRefAc {
    type Optioned = RoleRefAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::RoleRef {
    fn into_optioned(self) -> RoleRefAc {
        RoleRefAc {
            api_group: Some(crate::OptionableConvert::into_optioned(self.api_group)),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(value: RoleRefAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_group: crate::OptionableConvert::try_from_optioned(
                value
                    .api_group
                    .ok_or(crate::optionable::Error {
                        missing_field: "api_group",
                    })?,
            )?,
            kind: crate::OptionableConvert::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::optionable::Error {
                        missing_field: "kind",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: RoleRefAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.api_group {
            crate::OptionableConvert::merge(&mut self.api_group, other_value)?;
        }
        if let Some(other_value) = other.kind {
            crate::OptionableConvert::merge(&mut self.kind, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
