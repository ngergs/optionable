pub struct IngressBackendOpt {
    pub resource: <Option<
        ::k8s_openapi::api::core::v1::TypedLocalObjectReference,
    > as crate::Optionable>::Optioned,
    pub service: <Option<
        ::k8s_openapi::api::networking::v1::IngressServiceBackend,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressBackend {
    type Optioned = IngressBackendOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressBackendOpt {
    type Optioned = IngressBackendOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressBackend {
    fn into_optioned(self) -> IngressBackendOpt {
        IngressBackendOpt {
            resource: crate::OptionableConvert::into_optioned(self.resource),
            service: crate::OptionableConvert::into_optioned(self.service),
        }
    }
    fn try_from_optioned(
        value: IngressBackendOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            resource: crate::OptionableConvert::try_from_optioned(value.resource)?,
            service: crate::OptionableConvert::try_from_optioned(value.service)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressBackendOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.resource, other.resource)?;
        crate::OptionableConvert::merge(&mut self.service, other.service)?;
        Ok(())
    }
}
