pub struct RoleOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub rules: <Option<
        std::vec::Vec<::k8s_openapi::api::rbac::v1::PolicyRule>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::Role {
    type Optioned = RoleOpt;
}
#[automatically_derived]
impl crate::Optionable for RoleOpt {
    type Optioned = RoleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::Role {
    fn into_optioned(self) -> RoleOpt {
        RoleOpt {
            metadata: self.metadata,
            rules: crate::OptionableConvert::into_optioned(self.rules),
        }
    }
    fn try_from_optioned(value: RoleOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            rules: crate::OptionableConvert::try_from_optioned(value.rules)?,
        })
    }
    fn merge(&mut self, other: RoleOpt) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.rules, other.rules)?;
        Ok(())
    }
}
