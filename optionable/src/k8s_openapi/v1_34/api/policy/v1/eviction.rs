pub struct EvictionOpt {
    pub delete_options: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions,
    > as crate::Optionable>::Optioned,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::policy::v1::eviction::Eviction {
    type Optioned = EvictionOpt;
}
#[automatically_derived]
impl crate::Optionable for EvictionOpt {
    type Optioned = EvictionOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::policy::v1::eviction::Eviction {
    fn into_optioned(self) -> EvictionOpt {
        EvictionOpt {
            delete_options: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions,
            > as crate::OptionableConvert>::into_optioned(self.delete_options),
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
        }
    }
    fn try_from_optioned(value: EvictionOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            delete_options: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions,
            > as crate::OptionableConvert>::try_from_optioned(value.delete_options)?,
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: EvictionOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::DeleteOptions,
        > as crate::OptionableConvert>::merge(
            &mut self.delete_options,
            other.delete_options,
        )?;
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        Ok(())
    }
}
