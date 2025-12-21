#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamespaceStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::NamespaceCondition>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::NamespaceStatus {
    type Optioned = NamespaceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for NamespaceStatusAc {
    type Optioned = NamespaceStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::NamespaceStatus {
    fn into_optioned(self) -> NamespaceStatusAc {
        NamespaceStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            phase: crate::OptionableConvert::into_optioned(self.phase),
        }
    }
    fn try_from_optioned(value: NamespaceStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            phase: crate::OptionableConvert::try_from_optioned(value.phase)?,
        })
    }
    fn merge(&mut self, other: NamespaceStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        crate::OptionableConvert::merge(&mut self.phase, other.phase)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::NamespaceStatus>
for NamespaceStatusAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::NamespaceStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::NamespaceStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::NamespaceStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
