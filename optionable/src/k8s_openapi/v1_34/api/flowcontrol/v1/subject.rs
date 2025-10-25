pub struct SubjectOpt {
    pub group: <Option<
        ::k8s_openapi::api::flowcontrol::v1::GroupSubject,
    > as crate::Optionable>::Optioned,
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub service_account: <Option<
        ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject,
    > as crate::Optionable>::Optioned,
    pub user: <Option<
        ::k8s_openapi::api::flowcontrol::v1::UserSubject,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::Subject {
    type Optioned = SubjectOpt;
}
#[automatically_derived]
impl crate::Optionable for SubjectOpt {
    type Optioned = SubjectOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::flowcontrol::v1::Subject {
    fn into_optioned(self) -> SubjectOpt {
        SubjectOpt {
            group: <Option<
                ::k8s_openapi::api::flowcontrol::v1::GroupSubject,
            > as crate::OptionableConvert>::into_optioned(self.group),
            kind: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.kind,
                ),
            ),
            service_account: <Option<
                ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject,
            > as crate::OptionableConvert>::into_optioned(self.service_account),
            user: <Option<
                ::k8s_openapi::api::flowcontrol::v1::UserSubject,
            > as crate::OptionableConvert>::into_optioned(self.user),
        }
    }
    fn try_from_optioned(value: SubjectOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            group: <Option<
                ::k8s_openapi::api::flowcontrol::v1::GroupSubject,
            > as crate::OptionableConvert>::try_from_optioned(value.group)?,
            kind: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::optionable::Error {
                        missing_field: "kind",
                    })?,
            )?,
            service_account: <Option<
                ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject,
            > as crate::OptionableConvert>::try_from_optioned(value.service_account)?,
            user: <Option<
                ::k8s_openapi::api::flowcontrol::v1::UserSubject,
            > as crate::OptionableConvert>::try_from_optioned(value.user)?,
        })
    }
    fn merge(&mut self, other: SubjectOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::flowcontrol::v1::GroupSubject,
        > as crate::OptionableConvert>::merge(&mut self.group, other.group)?;
        if let Some(other_value) = other.kind {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.kind,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject,
        > as crate::OptionableConvert>::merge(
            &mut self.service_account,
            other.service_account,
        )?;
        <Option<
            ::k8s_openapi::api::flowcontrol::v1::UserSubject,
        > as crate::OptionableConvert>::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
