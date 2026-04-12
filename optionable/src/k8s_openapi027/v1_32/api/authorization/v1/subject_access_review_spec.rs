#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes and NonResourceAuthorizationAttributes must be set
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SubjectAccessReviewSpecAc {
    /// Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<
        std::collections::BTreeMap<
            std::string::String,
            std::vec::Vec<std::string::String>,
        >,
    >,
    /// Groups is the groups you're testing for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<std::vec::Vec<std::string::String>>,
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
    /// UID information about the requesting user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
    /// User is the user you're testing for. If you specify "User" but not "Groups", then is it interpreted as "What if User were not a member of any groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::authorization::v1::SubjectAccessReviewSpec {
    type Optioned = SubjectAccessReviewSpecAc;
}
#[automatically_derived]
impl crate::Optionable for SubjectAccessReviewSpecAc {
    type Optioned = SubjectAccessReviewSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authorization::v1::SubjectAccessReviewSpec {
    fn into_optioned(self) -> SubjectAccessReviewSpecAc {
        SubjectAccessReviewSpecAc {
            extra: self.extra,
            groups: self.groups,
            non_resource_attributes: crate::OptionableConvert::into_optioned(
                self.non_resource_attributes,
            ),
            resource_attributes: crate::OptionableConvert::into_optioned(
                self.resource_attributes,
            ),
            uid: self.uid,
            user: self.user,
        }
    }
    fn try_from_optioned(
        value: SubjectAccessReviewSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            extra: value.extra,
            groups: value.groups,
            non_resource_attributes: crate::OptionableConvert::try_from_optioned(
                value.non_resource_attributes,
            )?,
            resource_attributes: crate::OptionableConvert::try_from_optioned(
                value.resource_attributes,
            )?,
            uid: value.uid,
            user: value.user,
        })
    }
    fn merge(&mut self, other: SubjectAccessReviewSpecAc) -> Result<(), crate::Error> {
        if self.extra.is_none() {
            self.extra = crate::OptionableConvert::try_from_optioned(other.extra)?;
        } else if let Some(self_value) = self.extra.as_mut()
            && let Some(other_value) = other.extra
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.groups.is_none() {
            self.groups = crate::OptionableConvert::try_from_optioned(other.groups)?;
        } else if let Some(self_value) = self.groups.as_mut()
            && let Some(other_value) = other.groups
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.non_resource_attributes.is_none() {
            self.non_resource_attributes = crate::OptionableConvert::try_from_optioned(
                other.non_resource_attributes,
            )?;
        } else if let Some(self_value) = self.non_resource_attributes.as_mut()
            && let Some(other_value) = other.non_resource_attributes
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.resource_attributes.is_none() {
            self.resource_attributes = crate::OptionableConvert::try_from_optioned(
                other.resource_attributes,
            )?;
        } else if let Some(self_value) = self.resource_attributes.as_mut()
            && let Some(other_value) = other.resource_attributes
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.uid.is_none() {
            self.uid = crate::OptionableConvert::try_from_optioned(other.uid)?;
        } else if let Some(self_value) = self.uid.as_mut()
            && let Some(other_value) = other.uid
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
impl crate::OptionedConvert<
    k8s_openapi027::api::authorization::v1::SubjectAccessReviewSpec,
> for SubjectAccessReviewSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::authorization::v1::SubjectAccessReviewSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authorization::v1::SubjectAccessReviewSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authorization::v1::SubjectAccessReviewSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
