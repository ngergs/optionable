#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// UserInfo holds the information about the user needed to implement the user.Info interface.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserInfoAc {
    /// Any additional information provided by the authenticator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<
        std::collections::BTreeMap<
            std::string::String,
            std::vec::Vec<std::string::String>,
        >,
    >,
    /// The names of groups this user is a part of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<std::vec::Vec<std::string::String>>,
    /// A unique value that identifies this user across time. If this user is deleted and another user by the same name is added, they will have different UIDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
    /// The name that uniquely identifies this user among all active users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::authentication::v1::UserInfo {
    type Optioned = UserInfoAc;
}
#[automatically_derived]
impl crate::Optionable for UserInfoAc {
    type Optioned = UserInfoAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::authentication::v1::UserInfo {
    fn into_optioned(self) -> UserInfoAc {
        UserInfoAc {
            extra: self.extra,
            groups: self.groups,
            uid: self.uid,
            username: self.username,
        }
    }
    fn try_from_optioned(value: UserInfoAc) -> Result<Self, crate::Error> {
        Ok(Self {
            extra: value.extra,
            groups: value.groups,
            uid: value.uid,
            username: value.username,
        })
    }
    fn merge(&mut self, other: UserInfoAc) -> Result<(), crate::Error> {
        if other.extra.is_some() {
            self.extra = other.extra;
        }
        if other.groups.is_some() {
            self.groups = other.groups;
        }
        if other.uid.is_some() {
            self.uid = other.uid;
        }
        if other.username.is_some() {
            self.username = other.username;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::authentication::v1::UserInfo>
for UserInfoAc {
    fn from_optionable(
        value: k8s_openapi027::api::authentication::v1::UserInfo,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::authentication::v1::UserInfo, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authentication::v1::UserInfo,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
