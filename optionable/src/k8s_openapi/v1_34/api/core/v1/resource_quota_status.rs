pub struct ResourceQuotaStatusOpt {
    pub hard: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub used: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceQuotaStatus {
    type Optioned = ResourceQuotaStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceQuotaStatusOpt {
    type Optioned = ResourceQuotaStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceQuotaStatus {
    fn into_optioned(self) -> ResourceQuotaStatusOpt {
        ResourceQuotaStatusOpt {
            hard: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.hard),
            used: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.used),
        }
    }
    fn try_from_optioned(
        value: ResourceQuotaStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hard: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.hard)?,
            used: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.used)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceQuotaStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.hard, other.hard)?;
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.used, other.used)?;
        Ok(())
    }
}
