#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamespaceStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::NamespaceCondition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NamespaceStatus {
    type Optioned = NamespaceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for NamespaceStatusAc {
    type Optioned = NamespaceStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NamespaceStatus {
    fn into_optioned(self) -> NamespaceStatusAc {
        NamespaceStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            phase: crate::OptionableConvert::into_optioned(self.phase),
        }
    }
    fn try_from_optioned(
        value: NamespaceStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            phase: crate::OptionableConvert::try_from_optioned(value.phase)?,
        })
    }
    fn merge(
        &mut self,
        other: NamespaceStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(&mut self.phase, other.phase)?;
        Ok(())
    }
}
