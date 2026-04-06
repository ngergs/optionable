#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RawExtensionAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<<::k8s_openapi027::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::apimachinery::pkg::runtime::RawExtension {
    type Optioned = RawExtensionAc;
}
#[automatically_derived]
impl crate::Optionable for RawExtensionAc {
    type Optioned = RawExtensionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::runtime::RawExtension {
    fn into_optioned(self) -> RawExtensionAc {
        RawExtensionAc(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: RawExtensionAc) -> Result<Self, crate::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value.0.ok_or(crate::Error { missing_field: "0" })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: RawExtensionAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::apimachinery::pkg::runtime::RawExtension>
for RawExtensionAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::runtime::RawExtension,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::apimachinery::pkg::runtime::RawExtension, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::runtime::RawExtension,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
