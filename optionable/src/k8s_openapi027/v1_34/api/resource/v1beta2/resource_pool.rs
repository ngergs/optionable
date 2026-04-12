#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourcePool describes the pool that ResourceSlices belong to.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourcePoolAc {
    /// Generation tracks the change in a pool over time. Whenever a driver changes something about one or more of the resources in a pool, it must change the generation in all ResourceSlices which are part of that pool. Consumers of ResourceSlices should only consider resources from the pool with the highest generation number. The generation may be reset by drivers, which should be fine for consumers, assuming that all ResourceSlices in a pool are updated to match or deleted.
    ///
    /// Combined with ResourceSliceCount, this mechanism enables consumers to detect pools which are comprised of multiple ResourceSlices and are in an incomplete state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    /// Name is used to identify the pool. For node-local devices, this is often the node name, but this is not required.
    ///
    /// It must not be longer than 253 characters and must consist of one or more DNS sub-domains separated by slashes. This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// ResourceSliceCount is the total number of ResourceSlices in the pool at this generation number. Must be greater than zero.
    ///
    /// Consumers can use this to check whether they have seen all ResourceSlices belonging to the same pool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_slice_count: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta2::ResourcePool {
    type Optioned = ResourcePoolAc;
}
#[automatically_derived]
impl crate::Optionable for ResourcePoolAc {
    type Optioned = ResourcePoolAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1beta2::ResourcePool {
    fn into_optioned(self) -> ResourcePoolAc {
        ResourcePoolAc {
            generation: Some(self.generation),
            name: Some(self.name),
            resource_slice_count: Some(self.resource_slice_count),
        }
    }
    fn try_from_optioned(value: ResourcePoolAc) -> Result<Self, crate::Error> {
        Ok(Self {
            generation: value
                .generation
                .ok_or(crate::Error {
                    missing_field: "generation",
                })?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            resource_slice_count: value
                .resource_slice_count
                .ok_or(crate::Error {
                    missing_field: "resource_slice_count",
                })?,
        })
    }
    fn merge(&mut self, other: ResourcePoolAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.generation {
            self.generation = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.resource_slice_count {
            self.resource_slice_count = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta2::ResourcePool>
for ResourcePoolAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::ResourcePool,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta2::ResourcePool, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::ResourcePool,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
