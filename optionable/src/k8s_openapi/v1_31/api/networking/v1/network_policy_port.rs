#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NetworkPolicyPortAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_port: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::NetworkPolicyPort {
    type Optioned = NetworkPolicyPortAc;
}
#[automatically_derived]
impl crate::Optionable for NetworkPolicyPortAc {
    type Optioned = NetworkPolicyPortAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::NetworkPolicyPort {
    fn into_optioned(self) -> NetworkPolicyPortAc {
        NetworkPolicyPortAc {
            end_port: crate::OptionableConvert::into_optioned(self.end_port),
            port: crate::OptionableConvert::into_optioned(self.port),
            protocol: crate::OptionableConvert::into_optioned(self.protocol),
        }
    }
    fn try_from_optioned(value: NetworkPolicyPortAc) -> Result<Self, crate::Error> {
        Ok(Self {
            end_port: crate::OptionableConvert::try_from_optioned(value.end_port)?,
            port: crate::OptionableConvert::try_from_optioned(value.port)?,
            protocol: crate::OptionableConvert::try_from_optioned(value.protocol)?,
        })
    }
    fn merge(&mut self, other: NetworkPolicyPortAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.end_port, other.end_port)?;
        crate::OptionableConvert::merge(&mut self.port, other.port)?;
        crate::OptionableConvert::merge(&mut self.protocol, other.protocol)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::networking::v1::NetworkPolicyPort>
for NetworkPolicyPortAc {
    fn from_optionable(
        value: ::k8s_openapi::api::networking::v1::NetworkPolicyPort,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::networking::v1::NetworkPolicyPort, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::networking::v1::NetworkPolicyPort,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
