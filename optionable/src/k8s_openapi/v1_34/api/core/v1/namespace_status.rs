pub struct NamespaceStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::NamespaceCondition>,
    > as crate::Optionable>::Optioned,
    pub phase: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NamespaceStatus {
    type Optioned = NamespaceStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for NamespaceStatusOpt {
    type Optioned = NamespaceStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NamespaceStatus {
    fn into_optioned(self) -> NamespaceStatusOpt {
        NamespaceStatusOpt {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            phase: crate::OptionableConvert::into_optioned(self.phase),
        }
    }
    fn try_from_optioned(
        value: NamespaceStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            phase: crate::OptionableConvert::try_from_optioned(value.phase)?,
        })
    }
    fn merge(
        &mut self,
        other: NamespaceStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(&mut self.phase, other.phase)?;
        Ok(())
    }
}
