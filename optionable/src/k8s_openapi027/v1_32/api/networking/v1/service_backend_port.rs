#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ServiceBackendPort is the service port being referenced.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceBackendPortAc {
    /// name is the name of the port on the Service. This is a mutually exclusive setting with "Number".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// number is the numerical port number (e.g. 80) on the Service. This is a mutually exclusive setting with "Name".
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
        if self.name.is_none() {
            self.name = crate::OptionableConvert::try_from_optioned(other.name)?;
        } else if let Some(self_value) = self.name.as_mut()
            && let Some(other_value) = other.name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.number.is_none() {
            self.number = crate::OptionableConvert::try_from_optioned(other.number)?;
        } else if let Some(self_value) = self.number.as_mut()
            && let Some(other_value) = other.number
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
