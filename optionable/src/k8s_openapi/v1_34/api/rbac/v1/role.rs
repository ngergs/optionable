pub struct RoleOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub rules: <Option<
        std::vec::Vec<::k8s_openapi::api::rbac::v1::PolicyRule>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::role::Role {
    type Optioned = RoleOpt;
}
#[automatically_derived]
impl crate::Optionable for RoleOpt {
    type Optioned = RoleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::role::Role {
    fn into_optioned(self) -> RoleOpt {
        RoleOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            rules: <Option<
                std::vec::Vec<::k8s_openapi::api::rbac::v1::PolicyRule>,
            > as crate::OptionableConvert>::into_optioned(self.rules),
        }
    }
    fn try_from_optioned(value: RoleOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            rules: <Option<
                std::vec::Vec<::k8s_openapi::api::rbac::v1::PolicyRule>,
            > as crate::OptionableConvert>::try_from_optioned(value.rules)?,
        })
    }
    fn merge(&mut self, other: RoleOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<::k8s_openapi::api::rbac::v1::PolicyRule>,
        > as crate::OptionableConvert>::merge(&mut self.rules, other.rules)?;
        Ok(())
    }
}
