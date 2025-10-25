pub struct NamespaceSpecOpt {
    pub finalizers: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NamespaceSpec {
    type Optioned = NamespaceSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for NamespaceSpecOpt {
    type Optioned = NamespaceSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NamespaceSpec {
    fn into_optioned(self) -> NamespaceSpecOpt {
        NamespaceSpecOpt {
            finalizers: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.finalizers),
        }
    }
    fn try_from_optioned(
        value: NamespaceSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            finalizers: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.finalizers)?,
        })
    }
    fn merge(
        &mut self,
        other: NamespaceSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.finalizers, other.finalizers)?;
        Ok(())
    }
}
