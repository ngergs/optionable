#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct GRPCActionAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::GRPCAction {
    type Optioned = GRPCActionAc;
}
#[automatically_derived]
impl crate::Optionable for GRPCActionAc {
    type Optioned = GRPCActionAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::GRPCAction {
    fn into_optioned(self) -> GRPCActionAc {
        GRPCActionAc {
            port: Some(self.port),
            service: crate::OptionableConvert::into_optioned(self.service),
        }
    }
    fn try_from_optioned(value: GRPCActionAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            port: value
                .port
                .ok_or(crate::optionable::Error {
                    missing_field: "port",
                })?,
            service: crate::OptionableConvert::try_from_optioned(value.service)?,
        })
    }
    fn merge(&mut self, other: GRPCActionAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        crate::OptionableConvert::merge(&mut self.service, other.service)?;
        Ok(())
    }
}
