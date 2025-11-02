#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IngressPortStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressPortStatus {
    type Optioned = IngressPortStatusAc;
}
#[automatically_derived]
impl crate::Optionable for IngressPortStatusAc {
    type Optioned = IngressPortStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressPortStatus {
    fn into_optioned(self) -> IngressPortStatusAc {
        IngressPortStatusAc {
            error: crate::OptionableConvert::into_optioned(self.error),
            port: Some(self.port),
            protocol: Some(crate::OptionableConvert::into_optioned(self.protocol)),
        }
    }
    fn try_from_optioned(
        value: IngressPortStatusAc,
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
        other: IngressPortStatusAc,
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
