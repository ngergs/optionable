pub struct ResourceQuotaOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub spec: <Option<
        ::k8s_openapi::api::core::v1::ResourceQuotaSpec,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::api::core::v1::ResourceQuotaStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceQuota {
    type Optioned = ResourceQuotaOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceQuotaOpt {
    type Optioned = ResourceQuotaOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceQuota {
    fn into_optioned(self) -> ResourceQuotaOpt {
        ResourceQuotaOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            spec: <Option<
                ::k8s_openapi::api::core::v1::ResourceQuotaSpec,
            > as crate::OptionableConvert>::into_optioned(self.spec),
            status: <Option<
                ::k8s_openapi::api::core::v1::ResourceQuotaStatus,
            > as crate::OptionableConvert>::into_optioned(self.status),
        }
    }
    fn try_from_optioned(
        value: ResourceQuotaOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            spec: <Option<
                ::k8s_openapi::api::core::v1::ResourceQuotaSpec,
            > as crate::OptionableConvert>::try_from_optioned(value.spec)?,
            status: <Option<
                ::k8s_openapi::api::core::v1::ResourceQuotaStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.status)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceQuotaOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::core::v1::ResourceQuotaSpec,
        > as crate::OptionableConvert>::merge(&mut self.spec, other.spec)?;
        <Option<
            ::k8s_openapi::api::core::v1::ResourceQuotaStatus,
        > as crate::OptionableConvert>::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
