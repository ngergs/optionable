pub struct HTTPGetActionOpt {
    pub host: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub http_headers: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::HTTPHeader>,
    > as crate::Optionable>::Optioned,
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub port: Option<
        <::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString as crate::Optionable>::Optioned,
    >,
    pub scheme: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::HTTPGetAction {
    type Optioned = HTTPGetActionOpt;
}
#[automatically_derived]
impl crate::Optionable for HTTPGetActionOpt {
    type Optioned = HTTPGetActionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::HTTPGetAction {
    fn into_optioned(self) -> HTTPGetActionOpt {
        HTTPGetActionOpt {
            host: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.host),
            http_headers: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::HTTPHeader>,
            > as crate::OptionableConvert>::into_optioned(self.http_headers),
            path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.path),
            port: Some(
                <::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString as crate::OptionableConvert>::into_optioned(
                    self.port,
                ),
            ),
            scheme: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.scheme),
        }
    }
    fn try_from_optioned(
        value: HTTPGetActionOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            host: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.host)?,
            http_headers: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::HTTPHeader>,
            > as crate::OptionableConvert>::try_from_optioned(value.http_headers)?,
            path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.path)?,
            port: <::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString as crate::OptionableConvert>::try_from_optioned(
                value
                    .port
                    .ok_or(crate::optionable::Error {
                        missing_field: "port",
                    })?,
            )?,
            scheme: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.scheme)?,
        })
    }
    fn merge(
        &mut self,
        other: HTTPGetActionOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.host, other.host)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::HTTPHeader>,
        > as crate::OptionableConvert>::merge(
            &mut self.http_headers,
            other.http_headers,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.path, other.path)?;
        if let Some(other_value) = other.port {
            <::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString as crate::OptionableConvert>::merge(
                &mut self.port,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.scheme, other.scheme)?;
        Ok(())
    }
}
