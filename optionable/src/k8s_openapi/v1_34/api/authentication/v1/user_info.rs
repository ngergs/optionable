pub struct UserInfoAc {
    pub extra: <Option<
        std::collections::BTreeMap<
            std::string::String,
            std::vec::Vec<std::string::String>,
        >,
    > as crate::Optionable>::Optioned,
    pub groups: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub username: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::UserInfo {
    type Optioned = UserInfoAc;
}
#[automatically_derived]
impl crate::Optionable for UserInfoAc {
    type Optioned = UserInfoAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::authentication::v1::UserInfo {
    fn into_optioned(self) -> UserInfoAc {
        UserInfoAc {
            extra: crate::OptionableConvert::into_optioned(self.extra),
            groups: crate::OptionableConvert::into_optioned(self.groups),
            uid: crate::OptionableConvert::into_optioned(self.uid),
            username: crate::OptionableConvert::into_optioned(self.username),
        }
    }
    fn try_from_optioned(value: UserInfoAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            extra: crate::OptionableConvert::try_from_optioned(value.extra)?,
            groups: crate::OptionableConvert::try_from_optioned(value.groups)?,
            uid: crate::OptionableConvert::try_from_optioned(value.uid)?,
            username: crate::OptionableConvert::try_from_optioned(value.username)?,
        })
    }
    fn merge(&mut self, other: UserInfoAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.extra, other.extra)?;
        crate::OptionableConvert::merge(&mut self.groups, other.groups)?;
        crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        crate::OptionableConvert::merge(&mut self.username, other.username)?;
        Ok(())
    }
}
