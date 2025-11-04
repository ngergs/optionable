#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ParamKindAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::ParamKind {
    type Optioned = ParamKindAc;
}
#[automatically_derived]
impl crate::Optionable for ParamKindAc {
    type Optioned = ParamKindAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::ParamKind {
    fn into_optioned(self) -> ParamKindAc {
        ParamKindAc {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            kind: crate::OptionableConvert::into_optioned(self.kind),
        }
    }
    fn try_from_optioned(value: ParamKindAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            kind: crate::OptionableConvert::try_from_optioned(value.kind)?,
        })
    }
    fn merge(&mut self, other: ParamKindAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        crate::OptionableConvert::merge(&mut self.kind, other.kind)?;
        Ok(())
    }
}
