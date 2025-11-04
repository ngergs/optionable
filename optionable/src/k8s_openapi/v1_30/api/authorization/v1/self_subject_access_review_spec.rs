#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct SelfSubjectAccessReviewSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_attributes: <Option<
        ::k8s_openapi::api::authorization::v1::NonResourceAttributes,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_attributes: <Option<
        ::k8s_openapi::api::authorization::v1::ResourceAttributes,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::SelfSubjectAccessReviewSpec {
    type Optioned = SelfSubjectAccessReviewSpecAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectAccessReviewSpecAc {
    type Optioned = SelfSubjectAccessReviewSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SelfSubjectAccessReviewSpec {
    fn into_optioned(self) -> SelfSubjectAccessReviewSpecAc {
        SelfSubjectAccessReviewSpecAc {
            non_resource_attributes: crate::OptionableConvert::into_optioned(
                self.non_resource_attributes,
            ),
            resource_attributes: crate::OptionableConvert::into_optioned(
                self.resource_attributes,
            ),
        }
    }
    fn try_from_optioned(
        value: SelfSubjectAccessReviewSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            non_resource_attributes: crate::OptionableConvert::try_from_optioned(
                value.non_resource_attributes,
            )?,
            resource_attributes: crate::OptionableConvert::try_from_optioned(
                value.resource_attributes,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: SelfSubjectAccessReviewSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.non_resource_attributes,
            other.non_resource_attributes,
        )?;
        crate::OptionableConvert::merge(
            &mut self.resource_attributes,
            other.resource_attributes,
        )?;
        Ok(())
    }
}
