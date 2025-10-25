pub struct ServiceOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub spec: <Option<
        ::k8s_openapi::api::core::v1::ServiceSpec,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::api::core::v1::ServiceStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Service {
    type Optioned = ServiceOpt;
}
#[automatically_derived]
impl crate::Optionable for ServiceOpt {
    type Optioned = ServiceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Service {
    fn into_optioned(self) -> ServiceOpt {
        ServiceOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            spec: <Option<
                ::k8s_openapi::api::core::v1::ServiceSpec,
            > as crate::OptionableConvert>::into_optioned(self.spec),
            status: <Option<
                ::k8s_openapi::api::core::v1::ServiceStatus,
            > as crate::OptionableConvert>::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: ServiceOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            spec: <Option<
                ::k8s_openapi::api::core::v1::ServiceSpec,
            > as crate::OptionableConvert>::try_from_optioned(value.spec)?,
            status: <Option<
                ::k8s_openapi::api::core::v1::ServiceStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: ServiceOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::core::v1::ServiceSpec,
        > as crate::OptionableConvert>::merge(&mut self.spec, other.spec)?;
        <Option<
            ::k8s_openapi::api::core::v1::ServiceStatus,
        > as crate::OptionableConvert>::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
