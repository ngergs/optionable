pub struct SelfSubjectAccessReviewSpecOpt {
    pub non_resource_attributes: <Option<
        ::k8s_openapi::api::authorization::v1::NonResourceAttributes,
    > as crate::Optionable>::Optioned,
    pub resource_attributes: <Option<
        ::k8s_openapi::api::authorization::v1::ResourceAttributes,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::SelfSubjectAccessReviewSpec {
    type Optioned = SelfSubjectAccessReviewSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectAccessReviewSpecOpt {
    type Optioned = SelfSubjectAccessReviewSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SelfSubjectAccessReviewSpec {
    fn into_optioned(self) -> SelfSubjectAccessReviewSpecOpt {
        SelfSubjectAccessReviewSpecOpt {
            non_resource_attributes: <Option<
                ::k8s_openapi::api::authorization::v1::NonResourceAttributes,
            > as crate::OptionableConvert>::into_optioned(self.non_resource_attributes),
            resource_attributes: <Option<
                ::k8s_openapi::api::authorization::v1::ResourceAttributes,
            > as crate::OptionableConvert>::into_optioned(self.resource_attributes),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectAccessReviewSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            non_resource_attributes: <Option<
                ::k8s_openapi::api::authorization::v1::NonResourceAttributes,
            > as crate::OptionableConvert>::try_from_optioned(
                value.non_resource_attributes,
            )?,
            resource_attributes: <Option<
                ::k8s_openapi::api::authorization::v1::ResourceAttributes,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_attributes)?,
        })
    }
    fn merge(
        &mut self,
        other: SelfSubjectAccessReviewSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
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
        Ok(())
    }
}
