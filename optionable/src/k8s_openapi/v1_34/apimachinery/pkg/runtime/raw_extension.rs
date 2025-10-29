pub struct RawExtensionOpt(
    pub Option<<::k8s_openapi::serde_json::Value as crate::Optionable>::Optioned>,
);
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::runtime::RawExtension {
    type Optioned = RawExtensionOpt;
}
#[automatically_derived]
impl crate::Optionable for RawExtensionOpt {
    type Optioned = RawExtensionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::runtime::RawExtension {
    fn into_optioned(self) -> RawExtensionOpt {
        RawExtensionOpt(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(
        value: RawExtensionOpt,
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
    fn merge(&mut self, other: RawExtensionOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
