#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CustomResourceSubresources defines the status and scale subresources for CustomResources.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceSubresourcesAc {
    /// scale indicates the custom resource should serve a `/scale` subresource that returns an `autoscaling/v1` Scale object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale as crate::Optionable>::Optioned,
    >,
    /// status indicates the custom resource should serve a `/status` subresource. When enabled: 1. requests to the custom resource primary endpoint ignore changes to the `status` stanza of the object. 2. requests to the custom resource `/status` subresource ignore changes to anything other than the `status` stanza of the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceStatus as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources {
    type Optioned = CustomResourceSubresourcesAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceSubresourcesAc {
    type Optioned = CustomResourceSubresourcesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources {
    fn into_optioned(self) -> CustomResourceSubresourcesAc {
        CustomResourceSubresourcesAc {
            scale: crate::OptionableConvert::into_optioned(self.scale),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: CustomResourceSubresourcesAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            scale: crate::OptionableConvert::try_from_optioned(value.scale)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceSubresourcesAc,
    ) -> Result<(), crate::Error> {
        if self.scale.is_none() {
            self.scale = crate::OptionableConvert::try_from_optioned(other.scale)?;
        } else if let Some(self_value) = self.scale.as_mut()
            && let Some(other_value) = other.scale
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.status.is_none() {
            self.status = crate::OptionableConvert::try_from_optioned(other.status)?;
        } else if let Some(self_value) = self.status.as_mut()
            && let Some(other_value) = other.status
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources,
> for CustomResourceSubresourcesAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresources,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
