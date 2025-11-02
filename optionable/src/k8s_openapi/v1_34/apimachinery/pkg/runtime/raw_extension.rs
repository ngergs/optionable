#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawExtensionAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<<::k8s_openapi::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::runtime::RawExtension {
    type Optioned = RawExtensionAc;
}
#[automatically_derived]
impl crate::Optionable for RawExtensionAc {
    type Optioned = RawExtensionAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::runtime::RawExtension {
    fn into_optioned(self) -> RawExtensionAc {
        RawExtensionAc(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(
        value: RawExtensionAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value
                        .0
                        .ok_or(crate::optionable::Error {
                            missing_field: "0",
                        })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: RawExtensionAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
