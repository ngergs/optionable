pub struct IngressPortStatusOpt {
    pub error: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub port: Option<i32>,
    pub protocol: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressPortStatus {
    type Optioned = IngressPortStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressPortStatusOpt {
    type Optioned = IngressPortStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressPortStatus {
    fn into_optioned(self) -> IngressPortStatusOpt {
        IngressPortStatusOpt {
            error: crate::OptionableConvert::into_optioned(self.error),
            port: Some(self.port),
            protocol: Some(crate::OptionableConvert::into_optioned(self.protocol)),
        }
    }
    fn try_from_optioned(
        value: IngressPortStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            error: crate::OptionableConvert::try_from_optioned(value.error)?,
            port: value
                .port
                .ok_or(crate::optionable::Error {
                    missing_field: "port",
                })?,
            protocol: crate::OptionableConvert::try_from_optioned(
                value
                    .protocol
                    .ok_or(crate::optionable::Error {
                        missing_field: "protocol",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: IngressPortStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.error, other.error)?;
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        if let Some(other_value) = other.protocol {
            crate::OptionableConvert::merge(&mut self.protocol, other_value)?;
        }
        Ok(())
    }
}
