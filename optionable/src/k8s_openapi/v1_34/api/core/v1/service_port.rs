#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ServicePortAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_port: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_port: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ServicePort {
    type Optioned = ServicePortAc;
}
#[automatically_derived]
impl crate::Optionable for ServicePortAc {
    type Optioned = ServicePortAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ServicePort {
    fn into_optioned(self) -> ServicePortAc {
        ServicePortAc {
            app_protocol: crate::OptionableConvert::into_optioned(self.app_protocol),
            name: crate::OptionableConvert::into_optioned(self.name),
            node_port: crate::OptionableConvert::into_optioned(self.node_port),
            port: Some(self.port),
            protocol: crate::OptionableConvert::into_optioned(self.protocol),
            target_port: crate::OptionableConvert::into_optioned(self.target_port),
        }
    }
    fn try_from_optioned(
        value: ServicePortAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            app_protocol: crate::OptionableConvert::try_from_optioned(
                value.app_protocol,
            )?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            node_port: crate::OptionableConvert::try_from_optioned(value.node_port)?,
            port: value
                .port
                .ok_or(crate::optionable::Error {
                    missing_field: "port",
                })?,
            protocol: crate::OptionableConvert::try_from_optioned(value.protocol)?,
            target_port: crate::OptionableConvert::try_from_optioned(value.target_port)?,
        })
    }
    fn merge(&mut self, other: ServicePortAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.app_protocol, other.app_protocol)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.node_port, other.node_port)?;
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        crate::OptionableConvert::merge(&mut self.protocol, other.protocol)?;
        crate::OptionableConvert::merge(&mut self.target_port, other.target_port)?;
        Ok(())
    }
}
