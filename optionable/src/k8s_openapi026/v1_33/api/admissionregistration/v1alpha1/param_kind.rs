#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParamKindAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::admissionregistration::v1alpha1::ParamKind {
    type Optioned = ParamKindAc;
}
#[automatically_derived]
impl crate::Optionable for ParamKindAc {
    type Optioned = ParamKindAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::admissionregistration::v1alpha1::ParamKind {
    fn into_optioned(self) -> ParamKindAc {
        ParamKindAc {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            kind: crate::OptionableConvert::into_optioned(self.kind),
        }
    }
    fn try_from_optioned(value: ParamKindAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            kind: crate::OptionableConvert::try_from_optioned(value.kind)?,
        })
    }
    fn merge(&mut self, other: ParamKindAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        crate::OptionableConvert::merge(&mut self.kind, other.kind)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::admissionregistration::v1alpha1::ParamKind,
> for ParamKindAc {
    fn from_optionable(
        value: k8s_openapi026::api::admissionregistration::v1alpha1::ParamKind,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::admissionregistration::v1alpha1::ParamKind,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::admissionregistration::v1alpha1::ParamKind,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
