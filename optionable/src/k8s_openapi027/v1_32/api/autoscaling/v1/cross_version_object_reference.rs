#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CrossVersionObjectReference contains enough information to let you identify the referred resource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CrossVersionObjectReferenceAc {
    /// apiVersion is the API version of the referent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<std::string::String>,
    /// kind is the kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
    /// name is the name of the referent; More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::autoscaling::v1::CrossVersionObjectReference {
    type Optioned = CrossVersionObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for CrossVersionObjectReferenceAc {
    type Optioned = CrossVersionObjectReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v1::CrossVersionObjectReference {
    fn into_optioned(self) -> CrossVersionObjectReferenceAc {
        CrossVersionObjectReferenceAc {
            api_version: self.api_version,
            kind: Some(self.kind),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(
        value: CrossVersionObjectReferenceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: value.api_version,
            kind: value
                .kind
                .ok_or(crate::Error {
                    missing_field: "kind",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: CrossVersionObjectReferenceAc,
    ) -> Result<(), crate::Error> {
        if other.api_version.is_some() {
            self.api_version = other.api_version;
        }
        if let Some(other_value) = other.kind {
            self.kind = other_value;
        }
        if let Some(other_value) = other.name {
            self.name = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::autoscaling::v1::CrossVersionObjectReference,
> for CrossVersionObjectReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v1::CrossVersionObjectReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::autoscaling::v1::CrossVersionObjectReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v1::CrossVersionObjectReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
