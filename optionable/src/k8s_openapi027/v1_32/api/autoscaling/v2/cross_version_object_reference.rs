#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CrossVersionObjectReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::autoscaling::v2::CrossVersionObjectReference {
    type Optioned = CrossVersionObjectReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for CrossVersionObjectReferenceAc {
    type Optioned = CrossVersionObjectReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::autoscaling::v2::CrossVersionObjectReference {
    fn into_optioned(self) -> CrossVersionObjectReferenceAc {
        CrossVersionObjectReferenceAc {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            kind: Some(crate::OptionableConvert::into_optioned(self.kind)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: CrossVersionObjectReferenceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            kind: crate::OptionableConvert::try_from_optioned(
                value
                    .kind
                    .ok_or(crate::Error {
                        missing_field: "kind",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CrossVersionObjectReferenceAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        if let Some(other_value) = other.kind {
            crate::OptionableConvert::merge(&mut self.kind, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::autoscaling::v2::CrossVersionObjectReference,
> for CrossVersionObjectReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::autoscaling::v2::CrossVersionObjectReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::autoscaling::v2::CrossVersionObjectReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::autoscaling::v2::CrossVersionObjectReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
