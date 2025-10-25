pub struct ServicePortOpt {
    pub app_protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub node_port: <Option<i32> as crate::Optionable>::Optioned,
    pub port: Option<i32>,
    pub protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub target_port: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ServicePort {
    type Optioned = ServicePortOpt;
}
#[automatically_derived]
impl crate::Optionable for ServicePortOpt {
    type Optioned = ServicePortOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ServicePort {
    fn into_optioned(self) -> ServicePortOpt {
        ServicePortOpt {
            app_protocol: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.app_protocol),
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.name),
            node_port: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.node_port),
            port: Some(self.port),
            protocol: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.protocol),
            target_port: <Option<
                ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            > as crate::OptionableConvert>::into_optioned(self.target_port),
        }
    }
    fn try_from_optioned(
        value: ServicePortOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            app_protocol: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.app_protocol)?,
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.name)?,
            node_port: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.node_port)?,
            port: value
                .port
                .ok_or(crate::optionable::Error {
                    missing_field: "port",
                })?,
            protocol: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.protocol)?,
            target_port: <Option<
                ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            > as crate::OptionableConvert>::try_from_optioned(value.target_port)?,
        })
    }
    fn merge(&mut self, other: ServicePortOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.app_protocol,
            other.app_protocol,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.name, other.name)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.node_port, other.node_port)?;
        if let Some(other_value) = other.port {
            self.port = other_value;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.protocol, other.protocol)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        > as crate::OptionableConvert>::merge(&mut self.target_port, other.target_port)?;
        Ok(())
    }
}
