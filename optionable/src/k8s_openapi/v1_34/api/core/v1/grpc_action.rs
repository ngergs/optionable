pub struct GRPCActionOpt {
    pub port: Option<i32>,
    pub service: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::GRPCAction {
    type Optioned = GRPCActionOpt;
}
#[automatically_derived]
impl crate::Optionable for GRPCActionOpt {
    type Optioned = GRPCActionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::GRPCAction {
    fn into_optioned(self) -> GRPCActionOpt {
        GRPCActionOpt {
            port: Some(self.port),
            service: crate::OptionableConvert::into_optioned(self.service),
        }
    }
    fn try_from_optioned(
        value: GRPCActionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            port: value
                .port
                .ok_or(crate::optionable::Error {
                    missing_field: "port",
                })?,
            service: crate::OptionableConvert::try_from_optioned(value.service)?,
        })
    }
    fn merge(&mut self, other: GRPCActionOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        crate::OptionableConvert::merge(&mut self.service, other.service)?;
        Ok(())
    }
}
