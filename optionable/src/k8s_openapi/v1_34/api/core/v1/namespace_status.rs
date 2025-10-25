pub struct NamespaceStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::NamespaceCondition>,
    > as crate::Optionable>::Optioned,
    pub phase: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::namespace_status::NamespaceStatus {
    type Optioned = NamespaceStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for NamespaceStatusOpt {
    type Optioned = NamespaceStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::namespace_status::NamespaceStatus {
    fn into_optioned(self) -> NamespaceStatusOpt {
        NamespaceStatusOpt {
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::NamespaceCondition>,
            > as crate::OptionableConvert>::into_optioned(self.conditions),
            phase: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.phase),
        }
    }
    fn try_from_optioned(
        value: NamespaceStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::NamespaceCondition>,
            > as crate::OptionableConvert>::try_from_optioned(value.conditions)?,
            phase: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.phase)?,
        })
    }
    fn merge(
        &mut self,
        other: NamespaceStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::NamespaceCondition>,
        > as crate::OptionableConvert>::merge(&mut self.conditions, other.conditions)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.phase, other.phase)?;
        Ok(())
    }
}
