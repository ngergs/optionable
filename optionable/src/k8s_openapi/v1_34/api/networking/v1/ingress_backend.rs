pub struct IngressBackendOpt {
    pub resource: <Option<
        ::k8s_openapi::api::core::v1::TypedLocalObjectReference,
    > as crate::Optionable>::Optioned,
    pub service: <Option<
        ::k8s_openapi::api::networking::v1::IngressServiceBackend,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::networking::v1::ingress_backend::IngressBackend {
    type Optioned = IngressBackendOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressBackendOpt {
    type Optioned = IngressBackendOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::ingress_backend::IngressBackend {
    fn into_optioned(self) -> IngressBackendOpt {
        IngressBackendOpt {
            resource: <Option<
                ::k8s_openapi::api::core::v1::TypedLocalObjectReference,
            > as crate::OptionableConvert>::into_optioned(self.resource),
            service: <Option<
                ::k8s_openapi::api::networking::v1::IngressServiceBackend,
            > as crate::OptionableConvert>::into_optioned(self.service),
        }
    }
    fn try_from_optioned(
        value: IngressBackendOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            resource: <Option<
                ::k8s_openapi::api::core::v1::TypedLocalObjectReference,
            > as crate::OptionableConvert>::try_from_optioned(value.resource)?,
            service: <Option<
                ::k8s_openapi::api::networking::v1::IngressServiceBackend,
            > as crate::OptionableConvert>::try_from_optioned(value.service)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressBackendOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::TypedLocalObjectReference,
        > as crate::OptionableConvert>::merge(&mut self.resource, other.resource)?;
        <Option<
            ::k8s_openapi::api::networking::v1::IngressServiceBackend,
        > as crate::OptionableConvert>::merge(&mut self.service, other.service)?;
        Ok(())
    }
}
