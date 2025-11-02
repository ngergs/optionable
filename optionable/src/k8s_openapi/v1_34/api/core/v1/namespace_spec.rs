#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
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
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NamespaceSpec {
    fn into_optioned(self) -> NamespaceSpecAc {
        NamespaceSpecAc {
            finalizers: crate::OptionableConvert::into_optioned(self.finalizers),
        }
    }
    fn try_from_optioned(
        value: NamespaceSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            finalizers: crate::OptionableConvert::try_from_optioned(value.finalizers)?,
        })
    }
    fn merge(&mut self, other: NamespaceSpecAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.finalizers, other.finalizers)?;
        Ok(())
    }
}
