#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceClaimTemplateSpec contains the metadata and fields for a ResourceClaim.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceClaimTemplateSpecAc {
    /// ObjectMeta may contain labels and annotations that will be copied into the ResourceClaim when creating it. No other fields are allowed and will be rejected during validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    /// Spec for the ResourceClaim. The entire content is copied unchanged into the ResourceClaim that gets created from this template. The same fields as in a ResourceClaim are also valid here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi027::api::resource::v1::ResourceClaimSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1::ResourceClaimTemplateSpec {
    type Optioned = ResourceClaimTemplateSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimTemplateSpecAc {
    type Optioned = ResourceClaimTemplateSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1::ResourceClaimTemplateSpec {
    fn into_optioned(self) -> ResourceClaimTemplateSpecAc {
        ResourceClaimTemplateSpecAc {
            metadata: crate::OptionableConvert::into_optioned(self.metadata),
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimTemplateSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: crate::OptionableConvert::try_from_optioned(value.metadata)?,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceClaimTemplateSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.metadata, other.metadata)?;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1::ResourceClaimTemplateSpec>
for ResourceClaimTemplateSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1::ResourceClaimTemplateSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1::ResourceClaimTemplateSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1::ResourceClaimTemplateSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
