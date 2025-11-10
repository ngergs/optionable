#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct IngressServiceBackendAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: <Option<
        ::k8s_openapi::api::networking::v1::ServiceBackendPort,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressServiceBackend {
    type Optioned = IngressServiceBackendAc;
}
#[automatically_derived]
impl crate::Optionable for IngressServiceBackendAc {
    type Optioned = IngressServiceBackendAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::IngressServiceBackend {
    fn into_optioned(self) -> IngressServiceBackendAc {
        IngressServiceBackendAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            port: crate::OptionableConvert::into_optioned(self.port),
        }
    }
    fn try_from_optioned(value: IngressServiceBackendAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            port: crate::OptionableConvert::try_from_optioned(value.port)?,
        })
    }
    fn merge(&mut self, other: IngressServiceBackendAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.port, other.port)?;
        Ok(())
    }
}
