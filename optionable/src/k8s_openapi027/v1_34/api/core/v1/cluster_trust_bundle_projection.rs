#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClusterTrustBundleProjectionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ClusterTrustBundleProjection {
    type Optioned = ClusterTrustBundleProjectionAc;
}
#[automatically_derived]
impl crate::Optionable for ClusterTrustBundleProjectionAc {
    type Optioned = ClusterTrustBundleProjectionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ClusterTrustBundleProjection {
    fn into_optioned(self) -> ClusterTrustBundleProjectionAc {
        ClusterTrustBundleProjectionAc {
            label_selector: crate::OptionableConvert::into_optioned(self.label_selector),
            name: self.name,
            optional: self.optional,
            path: Some(self.path),
            signer_name: self.signer_name,
        }
    }
    fn try_from_optioned(
        value: ClusterTrustBundleProjectionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            label_selector: crate::OptionableConvert::try_from_optioned(
                value.label_selector,
            )?,
            name: value.name,
            optional: value.optional,
            path: value
                .path
                .ok_or(crate::Error {
                    missing_field: "path",
                })?,
            signer_name: value.signer_name,
        })
    }
    fn merge(
        &mut self,
        other: ClusterTrustBundleProjectionAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.label_selector, other.label_selector)?;
        self.name = other.name;
        self.optional = other.optional;
        if let Some(other_value) = other.path {
            self.path = other_value;
        }
        self.signer_name = other.signer_name;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ClusterTrustBundleProjection>
for ClusterTrustBundleProjectionAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ClusterTrustBundleProjection,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::ClusterTrustBundleProjection,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ClusterTrustBundleProjection,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
