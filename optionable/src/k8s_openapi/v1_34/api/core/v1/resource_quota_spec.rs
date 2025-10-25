pub struct ResourceQuotaSpecOpt {
    pub hard: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    pub scope_selector: <Option<
        ::k8s_openapi::api::core::v1::ScopeSelector,
    > as crate::Optionable>::Optioned,
    pub scopes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::resource_quota_spec::ResourceQuotaSpec {
    type Optioned = ResourceQuotaSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceQuotaSpecOpt {
    type Optioned = ResourceQuotaSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::resource_quota_spec::ResourceQuotaSpec {
    fn into_optioned(self) -> ResourceQuotaSpecOpt {
        ResourceQuotaSpecOpt {
            hard: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::into_optioned(self.hard),
            scope_selector: <Option<
                ::k8s_openapi::api::core::v1::ScopeSelector,
            > as crate::OptionableConvert>::into_optioned(self.scope_selector),
            scopes: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.scopes),
        }
    }
    fn try_from_optioned(
        value: ResourceQuotaSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hard: <Option<
                std::collections::BTreeMap<
                    std::string::String,
                    ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.hard)?,
            scope_selector: <Option<
                ::k8s_openapi::api::core::v1::ScopeSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.scope_selector)?,
            scopes: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.scopes)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceQuotaSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::collections::BTreeMap<
                std::string::String,
                ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
            >,
        > as crate::OptionableConvert>::merge(&mut self.hard, other.hard)?;
        <Option<
            ::k8s_openapi::api::core::v1::ScopeSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.scope_selector,
            other.scope_selector,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.scopes, other.scopes)?;
        Ok(())
    }
}
