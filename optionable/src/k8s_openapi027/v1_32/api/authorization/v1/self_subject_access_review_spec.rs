#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelfSubjectAccessReviewSpecAc {
    /// NonResourceAttributes describes information for a non-resource access request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_resource_attributes: Option<
        <::k8s_openapi027::api::authorization::v1::NonResourceAttributes as crate::Optionable>::Optioned,
    >,
    /// ResourceAuthorizationAttributes describes information for a resource access request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_attributes: Option<
        <::k8s_openapi027::api::authorization::v1::ResourceAttributes as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::authorization::v1::SelfSubjectAccessReviewSpec {
    type Optioned = SelfSubjectAccessReviewSpecAc;
}
#[automatically_derived]
impl crate::Optionable for SelfSubjectAccessReviewSpecAc {
    type Optioned = SelfSubjectAccessReviewSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authorization::v1::SelfSubjectAccessReviewSpec {
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
    ) -> Result<Self, crate::Error> {
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
    ) -> Result<(), crate::Error> {
        if self.non_resource_attributes.is_none() {
            self.non_resource_attributes = crate::OptionableConvert::try_from_optioned(
                other.non_resource_attributes,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.non_resource_attributes,
                other.non_resource_attributes,
            )?;
        }
        if self.resource_attributes.is_none() {
            self.resource_attributes = crate::OptionableConvert::try_from_optioned(
                other.resource_attributes,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.resource_attributes,
                other.resource_attributes,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::authorization::v1::SelfSubjectAccessReviewSpec,
> for SelfSubjectAccessReviewSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::authorization::v1::SelfSubjectAccessReviewSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authorization::v1::SelfSubjectAccessReviewSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authorization::v1::SelfSubjectAccessReviewSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
