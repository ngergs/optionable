pub struct UserInfoOpt {
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
impl crate::Optionable for ::k8s_openapi::api::authentication::v1::user_info::UserInfo {
    type Optioned = UserInfoOpt;
}
#[automatically_derived]
impl crate::Optionable for UserInfoOpt {
    type Optioned = UserInfoOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authentication::v1::user_info::UserInfo {
    fn into_optioned(self) -> UserInfoOpt {
        UserInfoOpt {
            extra: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    std::vec::Vec<std::string::String>,
                >,
            > as crate::OptionableConvert>::into_optioned(self.extra),
            groups: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.groups),
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.uid),
            username: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.username),
        }
    }
    fn try_from_optioned(value: UserInfoOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            extra: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    std::vec::Vec<std::string::String>,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.extra)?,
            groups: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.groups)?,
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.uid)?,
            username: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.username)?,
        })
    }
    fn merge(&mut self, other: UserInfoOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                std::vec::Vec<std::string::String>,
            >,
        > as crate::OptionableConvert>::merge(&mut self.extra, other.extra)?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.groups, other.groups)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.uid, other.uid)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.username, other.username)?;
        Ok(())
    }
}
