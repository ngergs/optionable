#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ParamKind is a tuple of Group Kind and Version.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParamKindAc {
    /// APIVersion is the API group version the resources belong to. In format of "group/version". Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<std::string::String>,
    /// Kind is the API kind the resources belong to. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::admissionregistration::v1::ParamKind {
    type Optioned = ParamKindAc;
}
#[automatically_derived]
impl crate::Optionable for ParamKindAc {
    type Optioned = ParamKindAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1::ParamKind {
    fn into_optioned(self) -> ParamKindAc {
        ParamKindAc {
            api_version: self.api_version,
            kind: self.kind,
        }
    }
    fn try_from_optioned(value: ParamKindAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: value.api_version,
            kind: value.kind,
        })
    }
    fn merge(&mut self, other: ParamKindAc) -> Result<(), crate::Error> {
        if self.api_version.is_none() {
            self.api_version = crate::OptionableConvert::try_from_optioned(
                other.api_version,
            )?;
        } else if let Some(self_value) = self.api_version.as_mut()
            && let Some(other_value) = other.api_version
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.kind.is_none() {
            self.kind = crate::OptionableConvert::try_from_optioned(other.kind)?;
        } else if let Some(self_value) = self.kind.as_mut()
            && let Some(other_value) = other.kind
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::admissionregistration::v1::ParamKind>
for ParamKindAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1::ParamKind,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1::ParamKind,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1::ParamKind,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for ParamKindAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.api_version, other.api_version);
        k8s_openapi027::DeepMerge::merge_from(&mut self.kind, other.kind);
    }
}
