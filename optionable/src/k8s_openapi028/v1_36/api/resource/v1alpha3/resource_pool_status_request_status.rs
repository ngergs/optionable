#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourcePoolStatusRequestStatus contains the calculated pool status information.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourcePoolStatusRequestStatusAc {
    /// Conditions provide information about the state of the request. A condition with type=Complete or type=Failed will always be set when the status is populated.
    ///
    /// Known condition types: - "Complete": True when the request has been processed successfully - "Failed": True when the request could not be processed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi028::apimachinery::pkg::apis::meta::v1::Condition as crate::Optionable>::Optioned,
        >,
    >,
    /// PoolCount is the total number of pools that matched the filter criteria, regardless of truncation. This helps users understand how many pools exist even when the response is truncated. A value of 0 means no pools matched the filter criteria.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_count: Option<i32>,
    /// Pools contains the first `spec.limit` matching pools, sorted by driver then pool name. If `len(pools) \< poolCount`, the list was truncated. When omitted, no pools matched the request filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pools: Option<
        std::vec::Vec<
            <::k8s_openapi028::api::resource::v1alpha3::PoolStatus as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestStatus {
    type Optioned = ResourcePoolStatusRequestStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourcePoolStatusRequestStatusAc {
    type Optioned = ResourcePoolStatusRequestStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestStatus {
    fn into_optioned(self) -> ResourcePoolStatusRequestStatusAc {
        ResourcePoolStatusRequestStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            pool_count: Some(self.pool_count),
            pools: crate::OptionableConvert::into_optioned(self.pools),
        }
    }
    fn try_from_optioned(
        value: ResourcePoolStatusRequestStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            pool_count: value
                .pool_count
                .ok_or(crate::Error {
                    missing_field: "pool_count",
                })?,
            pools: crate::OptionableConvert::try_from_optioned(value.pools)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourcePoolStatusRequestStatusAc,
    ) -> Result<(), crate::Error> {
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.pool_count {
            self.pool_count = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.pools.is_none() {
            self.pools = crate::OptionableConvert::try_from_optioned(other.pools)?;
        } else if let Some(self_value) = self.pools.as_mut()
            && let Some(other_value) = other.pools
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestStatus,
> for ResourcePoolStatusRequestStatusAc {
    fn from_optionable(
        value: k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for ResourcePoolStatusRequestStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.conditions, other.conditions);
        k8s_openapi028::DeepMerge::merge_from(&mut self.pool_count, other.pool_count);
        k8s_openapi028::DeepMerge::merge_from(&mut self.pools, other.pools);
    }
}
