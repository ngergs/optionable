#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// Subject matches the originator of a request, as identified by the request authentication system. There are three ways of matching an originator; by user, group, or service account.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SubjectAc {
    /// `group` matches based on user group name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<
        <::k8s_openapi027::api::flowcontrol::v1::GroupSubject as crate::Optionable>::Optioned,
    >,
    /// `kind` indicates which one of the other fields is non-empty. Required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// `serviceAccount` matches ServiceAccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account: Option<
        <::k8s_openapi027::api::flowcontrol::v1::ServiceAccountSubject as crate::Optionable>::Optioned,
    >,
    /// `user` matches based on username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<
        <::k8s_openapi027::api::flowcontrol::v1::UserSubject as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::flowcontrol::v1::Subject {
    type Optioned = SubjectAc;
}
#[automatically_derived]
impl crate::Optionable for SubjectAc {
    type Optioned = SubjectAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::flowcontrol::v1::Subject {
    fn into_optioned(self) -> SubjectAc {
        SubjectAc {
            group: crate::OptionableConvert::into_optioned(self.group),
            kind: Some(self.kind),
            service_account: crate::OptionableConvert::into_optioned(
                self.service_account,
            ),
            user: crate::OptionableConvert::into_optioned(self.user),
        }
    }
    fn try_from_optioned(value: SubjectAc) -> Result<Self, crate::Error> {
        Ok(Self {
            group: crate::OptionableConvert::try_from_optioned(value.group)?,
            kind: value
                .kind
                .ok_or(crate::Error {
                    missing_field: "kind",
                })?,
            service_account: crate::OptionableConvert::try_from_optioned(
                value.service_account,
            )?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
        })
    }
    fn merge(&mut self, other: SubjectAc) -> Result<(), crate::Error> {
        if self.group.is_none() {
            self.group = crate::OptionableConvert::try_from_optioned(other.group)?;
        } else if let Some(self_value) = self.group.as_mut()
            && let Some(other_value) = other.group
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.kind {
            self.kind = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.service_account.is_none() {
            self.service_account = crate::OptionableConvert::try_from_optioned(
                other.service_account,
            )?;
        } else if let Some(self_value) = self.service_account.as_mut()
            && let Some(other_value) = other.service_account
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.user.is_none() {
            self.user = crate::OptionableConvert::try_from_optioned(other.user)?;
        } else if let Some(self_value) = self.user.as_mut()
            && let Some(other_value) = other.user
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::flowcontrol::v1::Subject>
for SubjectAc {
    fn from_optionable(value: k8s_openapi027::api::flowcontrol::v1::Subject) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::flowcontrol::v1::Subject, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::Subject,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for SubjectAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.group, other.group);
        k8s_openapi027::DeepMerge::merge_from(&mut self.kind, other.kind);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.service_account,
            other.service_account,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.user, other.user);
    }
}
