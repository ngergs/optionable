#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceQuotaSpec defines the desired hard limits to enforce for Quota.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceQuotaSpecAc {
    /// hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: Option<
        std::collections::BTreeMap<
            std::string::String,
            <::k8s_openapi027::apimachinery::pkg::api::resource::Quantity as crate::Optionable>::Optioned,
        >,
    >,
    /// scopeSelector is also a collection of filters like scopes that must match each object tracked by a quota but expressed using ScopeSelectorOperator in combination with possible values. For a resource to match, both scopes AND scopeSelector (if specified in spec), must be matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_selector: Option<
        <::k8s_openapi027::api::core::v1::ScopeSelector as crate::Optionable>::Optioned,
    >,
    /// A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ResourceQuotaSpec {
    type Optioned = ResourceQuotaSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceQuotaSpecAc {
    type Optioned = ResourceQuotaSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ResourceQuotaSpec {
    fn into_optioned(self) -> ResourceQuotaSpecAc {
        ResourceQuotaSpecAc {
            hard: crate::OptionableConvert::into_optioned(self.hard),
            scope_selector: crate::OptionableConvert::into_optioned(self.scope_selector),
            scopes: self.scopes,
        }
    }
    fn try_from_optioned(value: ResourceQuotaSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hard: crate::OptionableConvert::try_from_optioned(value.hard)?,
            scope_selector: crate::OptionableConvert::try_from_optioned(
                value.scope_selector,
            )?,
            scopes: value.scopes,
        })
    }
    fn merge(&mut self, other: ResourceQuotaSpecAc) -> Result<(), crate::Error> {
        if self.hard.is_none() {
            self.hard = crate::OptionableConvert::try_from_optioned(other.hard)?;
        } else {
            crate::OptionableConvert::merge(&mut self.hard, other.hard)?;
        }
        if self.scope_selector.is_none() {
            self.scope_selector = crate::OptionableConvert::try_from_optioned(
                other.scope_selector,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.scope_selector,
                other.scope_selector,
            )?;
        }
        if self.scopes.is_none() {
            self.scopes = crate::OptionableConvert::try_from_optioned(other.scopes)?;
        } else {
            self.scopes = crate::OptionableConvert::try_from_optioned(other.scopes)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ResourceQuotaSpec>
for ResourceQuotaSpecAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ResourceQuotaSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ResourceQuotaSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ResourceQuotaSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
