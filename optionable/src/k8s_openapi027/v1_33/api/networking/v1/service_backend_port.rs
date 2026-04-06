#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceBackendPortAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::ServiceBackendPort {
    type Optioned = ServiceBackendPortAc;
}
#[automatically_derived]
impl crate::Optionable for ServiceBackendPortAc {
    type Optioned = ServiceBackendPortAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::networking::v1::ServiceBackendPort {
    fn into_optioned(self) -> ServiceBackendPortAc {
        ServiceBackendPortAc {
            name: self.name,
            number: self.number,
        }
    }
    fn try_from_optioned(value: ServiceBackendPortAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            number: value.number,
        })
    }
    fn merge(&mut self, other: ServiceBackendPortAc) -> Result<(), crate::Error> {
        self.name = other.name;
        self.number = other.number;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::ServiceBackendPort>
for ServiceBackendPortAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::ServiceBackendPort,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::ServiceBackendPort, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::ServiceBackendPort,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
