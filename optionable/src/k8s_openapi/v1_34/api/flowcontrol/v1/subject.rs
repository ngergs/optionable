#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct SubjectAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: <Option<
        ::k8s_openapi::api::flowcontrol::v1::GroupSubject,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: <Option<
        ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: <Option<
        ::k8s_openapi::api::flowcontrol::v1::UserSubject,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::Subject {
    type Optioned = SubjectAc;
}
#[automatically_derived]
impl crate::Optionable for SubjectAc {
    type Optioned = SubjectAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::flowcontrol::v1::Subject {
    fn into_optioned(self) -> SubjectAc {
        SubjectAc {
            group: crate::OptionableConvert::into_optioned(self.group),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            service_account: crate::OptionableConvert::into_optioned(
                self.service_account,
            ),
            user: crate::OptionableConvert::into_optioned(self.user),
        }
    }
    fn try_from_optioned(value: SubjectAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group: crate::OptionableConvert::try_from_optioned(value.group)?,
            kind: crate::OptionableConvert::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::Error {
                        missing_field: "kind",
                    })?,
            )?,
            service_account: crate::OptionableConvert::try_from_optioned(
                value.service_account,
            )?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
        })
    }
    fn merge(&mut self, other: SubjectAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.group, other.group)?;
        if let Some(other_value) = other.kind {
            crate::OptionableConvert::merge(&mut self.kind, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.service_account,
            other.service_account,
        )?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
