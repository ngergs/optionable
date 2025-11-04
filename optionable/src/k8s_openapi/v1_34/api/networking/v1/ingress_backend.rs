#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct IngressBackendAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: <Option<
        ::k8s_openapi::api::core::v1::TypedLocalObjectReference,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: <Option<
        ::k8s_openapi::api::networking::v1::IngressServiceBackend,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressBackend {
    type Optioned = IngressBackendAc;
}
#[automatically_derived]
impl crate::Optionable for IngressBackendAc {
    type Optioned = IngressBackendAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressBackend {
    fn into_optioned(self) -> IngressBackendAc {
        IngressBackendAc {
            resource: crate::OptionableConvert::into_optioned(self.resource),
            service: crate::OptionableConvert::into_optioned(self.service),
        }
    }
    fn try_from_optioned(
        value: IngressBackendAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            resource: crate::OptionableConvert::try_from_optioned(value.resource)?,
            service: crate::OptionableConvert::try_from_optioned(value.service)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressBackendAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.resource, other.resource)?;
        crate::OptionableConvert::merge(&mut self.service, other.service)?;
        Ok(())
    }
}
