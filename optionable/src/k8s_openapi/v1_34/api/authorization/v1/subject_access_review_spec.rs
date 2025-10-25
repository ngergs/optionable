pub struct SubjectAccessReviewSpecOpt {
    pub extra: <Option<
        std::collections::BTreeMap<
            std::string::String,
            std::vec::Vec<std::string::String>,
        >,
    > as crate::Optionable>::Optioned,
    pub groups: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub non_resource_attributes: <Option<
        ::k8s_openapi::api::authorization::v1::NonResourceAttributes,
    > as crate::Optionable>::Optioned,
    pub resource_attributes: <Option<
        ::k8s_openapi::api::authorization::v1::ResourceAttributes,
    > as crate::Optionable>::Optioned,
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub user: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::SubjectAccessReviewSpec {
    type Optioned = SubjectAccessReviewSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for SubjectAccessReviewSpecOpt {
    type Optioned = SubjectAccessReviewSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SubjectAccessReviewSpec {
    fn into_optioned(self) -> SubjectAccessReviewSpecOpt {
        SubjectAccessReviewSpecOpt {
            extra: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    std::vec::Vec<std::string::String>,
                >,
            > as crate::OptionableConvert>::into_optioned(self.extra),
            groups: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.groups),
            non_resource_attributes: <Option<
                ::k8s_openapi::api::authorization::v1::NonResourceAttributes,
            > as crate::OptionableConvert>::into_optioned(self.non_resource_attributes),
            resource_attributes: <Option<
                ::k8s_openapi::api::authorization::v1::ResourceAttributes,
            > as crate::OptionableConvert>::into_optioned(self.resource_attributes),
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.uid),
            user: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.user),
        }
    }
    fn try_from_optioned(
        value: SubjectAccessReviewSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
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
            non_resource_attributes: <Option<
                ::k8s_openapi::api::authorization::v1::NonResourceAttributes,
            > as crate::OptionableConvert>::try_from_optioned(
                value.non_resource_attributes,
            )?,
            resource_attributes: <Option<
                ::k8s_openapi::api::authorization::v1::ResourceAttributes,
            > as crate::OptionableConvert>::try_from_optioned(
                value.resource_attributes,
            )?,
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.uid)?,
            user: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.user)?,
        })
    }
    fn merge(
        &mut self,
        other: SubjectAccessReviewSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
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
            ::k8s_openapi::api::authorization::v1::NonResourceAttributes,
        > as crate::OptionableConvert>::merge(
            &mut self.non_resource_attributes,
            other.non_resource_attributes,
        )?;
        <Option<
            ::k8s_openapi::api::authorization::v1::ResourceAttributes,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_attributes,
            other.resource_attributes,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.uid, other.uid)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
