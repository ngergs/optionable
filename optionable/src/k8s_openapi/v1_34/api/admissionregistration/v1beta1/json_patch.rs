#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JSONPatchAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1beta1::JSONPatch {
    type Optioned = JSONPatchAc;
}
#[automatically_derived]
impl crate::Optionable for JSONPatchAc {
    type Optioned = JSONPatchAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1beta1::JSONPatch {
    fn into_optioned(self) -> JSONPatchAc {
        JSONPatchAc {
            expression: crate::OptionableConvert::into_optioned(self.expression),
        }
    }
    fn try_from_optioned(value: JSONPatchAc) -> Result<Self, crate::Error> {
        Ok(Self {
            expression: crate::OptionableConvert::try_from_optioned(value.expression)?,
        })
    }
    fn merge(&mut self, other: JSONPatchAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.expression, other.expression)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::admissionregistration::v1beta1::JSONPatch,
> for JSONPatchAc {
    fn from_optionable(
        value: ::k8s_openapi::api::admissionregistration::v1beta1::JSONPatch,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::admissionregistration::v1beta1::JSONPatch,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::admissionregistration::v1beta1::JSONPatch,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
