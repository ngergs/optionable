pub struct RoleBindingOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub role_ref: Option<
        <::k8s_openapi::api::rbac::v1::RoleRef as crate::Optionable>::Optioned,
    >,
    pub subjects: <Option<
        std::vec::Vec<::k8s_openapi::api::rbac::v1::Subject>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::rbac::v1::RoleBinding {
    type Optioned = RoleBindingOpt;
}
#[automatically_derived]
impl crate::Optionable for RoleBindingOpt {
    type Optioned = RoleBindingOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::rbac::v1::RoleBinding {
    fn into_optioned(self) -> RoleBindingOpt {
        RoleBindingOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            role_ref: Some(
                <::k8s_openapi::api::rbac::v1::RoleRef as crate::OptionableConvert>::into_optioned(
                    self.role_ref,
                ),
            ),
            subjects: <Option<
                std::vec::Vec<::k8s_openapi::api::rbac::v1::Subject>,
            > as crate::OptionableConvert>::into_optioned(self.subjects),
        }
    }
    fn try_from_optioned(
        value: RoleBindingOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            role_ref: <::k8s_openapi::api::rbac::v1::RoleRef as crate::OptionableConvert>::try_from_optioned(
                value
                    .role_ref
                    .ok_or(crate::optionable::Error {
                        missing_field: "role_ref",
                    })?,
            )?,
            subjects: <Option<
                std::vec::Vec<::k8s_openapi::api::rbac::v1::Subject>,
            > as crate::OptionableConvert>::try_from_optioned(value.subjects)?,
        })
    }
    fn merge(&mut self, other: RoleBindingOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        if let Some(other_value) = other.role_ref {
            <::k8s_openapi::api::rbac::v1::RoleRef as crate::OptionableConvert>::merge(
                &mut self.role_ref,
                other_value,
            )?;
        }
        <Option<
            std::vec::Vec<::k8s_openapi::api::rbac::v1::Subject>,
        > as crate::OptionableConvert>::merge(&mut self.subjects, other.subjects)?;
        Ok(())
    }
}
