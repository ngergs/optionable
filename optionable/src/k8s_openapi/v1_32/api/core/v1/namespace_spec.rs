#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalizers: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NamespaceSpec {
    type Optioned = NamespaceSpecAc;
}
#[automatically_derived]
impl crate::Optionable for NamespaceSpecAc {
    type Optioned = NamespaceSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NamespaceSpec {
    fn into_optioned(self) -> NamespaceSpecAc {
        NamespaceSpecAc {
            finalizers: crate::OptionableConvert::into_optioned(self.finalizers),
        }
    }
    fn try_from_optioned(value: NamespaceSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            finalizers: crate::OptionableConvert::try_from_optioned(value.finalizers)?,
        })
    }
    fn merge(&mut self, other: NamespaceSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.finalizers, other.finalizers)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::NamespaceSpec>
for NamespaceSpecAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::NamespaceSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::NamespaceSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::NamespaceSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
