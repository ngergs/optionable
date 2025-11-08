#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_selector: <Option<
        ::k8s_openapi::api::core::v1::ScopeSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceQuotaSpec {
    type Optioned = ResourceQuotaSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceQuotaSpecAc {
    type Optioned = ResourceQuotaSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceQuotaSpec {
    fn into_optioned(self) -> ResourceQuotaSpecAc {
        ResourceQuotaSpecAc {
            hard: crate::OptionableConvert::into_optioned(self.hard),
            scope_selector: crate::OptionableConvert::into_optioned(self.scope_selector),
            scopes: crate::OptionableConvert::into_optioned(self.scopes),
        }
    }
    fn try_from_optioned(value: ResourceQuotaSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hard: crate::OptionableConvert::try_from_optioned(value.hard)?,
            scope_selector: crate::OptionableConvert::try_from_optioned(
                value.scope_selector,
            )?,
            scopes: crate::OptionableConvert::try_from_optioned(value.scopes)?,
        })
    }
    fn merge(&mut self, other: ResourceQuotaSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.hard, other.hard)?;
        crate::OptionableConvert::merge(&mut self.scope_selector, other.scope_selector)?;
        crate::OptionableConvert::merge(&mut self.scopes, other.scopes)?;
        Ok(())
    }
}
