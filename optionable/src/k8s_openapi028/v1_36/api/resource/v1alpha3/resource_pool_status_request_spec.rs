#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourcePoolStatusRequestSpec defines the filters for the pool status request.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourcePoolStatusRequestSpecAc {
    /// Driver specifies the DRA driver name to filter pools. Only pools from ResourceSlices with this driver will be included. Must be a DNS subdomain (e.g., "gpu.example.com").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<std::string::String>,
    /// Limit optionally specifies the maximum number of pools to return in the status. If more pools match the filter criteria, the response will be truncated (i.e., len(status.pools) \< status.poolCount).
    ///
    /// Default: 100 Minimum: 1 Maximum: 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// PoolName optionally filters to a specific pool name. If not specified, all pools from the specified driver are included. When specified, must be a non-empty valid resource pool name (DNS subdomains separated by "/").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestSpec {
    type Optioned = ResourcePoolStatusRequestSpecAc;
}
#[automatically_derived]
impl crate::Optionable for ResourcePoolStatusRequestSpecAc {
    type Optioned = ResourcePoolStatusRequestSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestSpec {
    fn into_optioned(self) -> ResourcePoolStatusRequestSpecAc {
        ResourcePoolStatusRequestSpecAc {
            driver: Some(self.driver),
            limit: self.limit,
            pool_name: self.pool_name,
        }
    }
    fn try_from_optioned(
        value: ResourcePoolStatusRequestSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            driver: value
                .driver
                .ok_or(crate::Error {
                    missing_field: "driver",
                })?,
            limit: value.limit,
            pool_name: value.pool_name,
        })
    }
    fn merge(
        &mut self,
        other: ResourcePoolStatusRequestSpecAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.driver {
            self.driver = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.limit.is_none() {
            self.limit = crate::OptionableConvert::try_from_optioned(other.limit)?;
        } else if let Some(self_value) = self.limit.as_mut()
            && let Some(other_value) = other.limit
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.pool_name.is_none() {
            self.pool_name = crate::OptionableConvert::try_from_optioned(
                other.pool_name,
            )?;
        } else if let Some(self_value) = self.pool_name.as_mut()
            && let Some(other_value) = other.pool_name
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestSpec,
> for ResourcePoolStatusRequestSpecAc {
    fn from_optionable(
        value: k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi028::api::resource::v1alpha3::ResourcePoolStatusRequestSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi028::DeepMerge for ResourcePoolStatusRequestSpecAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi028::DeepMerge::merge_from(&mut self.driver, other.driver);
        k8s_openapi028::DeepMerge::merge_from(&mut self.limit, other.limit);
        k8s_openapi028::DeepMerge::merge_from(&mut self.pool_name, other.pool_name);
    }
}
