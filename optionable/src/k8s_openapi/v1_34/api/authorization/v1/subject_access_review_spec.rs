pub struct SubjectAccessReviewSpecAc {
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
    type Optioned = SubjectAccessReviewSpecAc;
}
#[automatically_derived]
impl crate::Optionable for SubjectAccessReviewSpecAc {
    type Optioned = SubjectAccessReviewSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SubjectAccessReviewSpec {
    fn into_optioned(self) -> SubjectAccessReviewSpecAc {
        SubjectAccessReviewSpecAc {
            extra: crate::OptionableConvert::into_optioned(self.extra),
            groups: crate::OptionableConvert::into_optioned(self.groups),
            non_resource_attributes: crate::OptionableConvert::into_optioned(
                self.non_resource_attributes,
            ),
            resource_attributes: crate::OptionableConvert::into_optioned(
                self.resource_attributes,
            ),
            uid: crate::OptionableConvert::into_optioned(self.uid),
            user: crate::OptionableConvert::into_optioned(self.user),
        }
    }
    fn try_from_optioned(
        value: SubjectAccessReviewSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            extra: crate::OptionableConvert::try_from_optioned(value.extra)?,
            groups: crate::OptionableConvert::try_from_optioned(value.groups)?,
            non_resource_attributes: crate::OptionableConvert::try_from_optioned(
                value.non_resource_attributes,
            )?,
            resource_attributes: crate::OptionableConvert::try_from_optioned(
                value.resource_attributes,
            )?,
            uid: crate::OptionableConvert::try_from_optioned(value.uid)?,
            user: crate::OptionableConvert::try_from_optioned(value.user)?,
        })
    }
    fn merge(
        &mut self,
        other: SubjectAccessReviewSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.extra, other.extra)?;
        crate::OptionableConvert::merge(&mut self.groups, other.groups)?;
        crate::OptionableConvert::merge(
            &mut self.non_resource_attributes,
            other.non_resource_attributes,
        )?;
        crate::OptionableConvert::merge(
            &mut self.resource_attributes,
            other.resource_attributes,
        )?;
        crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        crate::OptionableConvert::merge(&mut self.user, other.user)?;
        Ok(())
    }
}
