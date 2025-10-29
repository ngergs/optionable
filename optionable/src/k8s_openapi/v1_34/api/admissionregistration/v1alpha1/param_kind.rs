pub struct ParamKindOpt {
    pub api_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub kind: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::ParamKind {
    type Optioned = ParamKindOpt;
}
#[automatically_derived]
impl crate::Optionable for ParamKindOpt {
    type Optioned = ParamKindOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::ParamKind {
    fn into_optioned(self) -> ParamKindOpt {
        ParamKindOpt {
            api_version: crate::OptionableConvert::into_optioned(self.api_version),
            kind: crate::OptionableConvert::into_optioned(self.kind),
        }
    }
    fn try_from_optioned(value: ParamKindOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_version: crate::OptionableConvert::try_from_optioned(value.api_version)?,
            kind: crate::OptionableConvert::try_from_optioned(value.kind)?,
        })
    }
    fn merge(&mut self, other: ParamKindOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_version, other.api_version)?;
        crate::OptionableConvert::merge(&mut self.kind, other.kind)?;
        Ok(())
    }
}
