#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UserInfoAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<
        std::collections::BTreeMap<
            std::string::String,
            std::vec::Vec<std::string::String>,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
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
        self.extra = other.extra;
        self.groups = other.groups;
        self.uid = other.uid;
        self.username = other.username;
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
