#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ResourcePoolAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_slice_count: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1::ResourcePool {
    type Optioned = ResourcePoolAc;
}
#[automatically_derived]
impl crate::Optionable for ResourcePoolAc {
    type Optioned = ResourcePoolAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1::ResourcePool {
    fn into_optioned(self) -> ResourcePoolAc {
        ResourcePoolAc {
            generation: Some(self.generation),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
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
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            resource_slice_count: value
                .resource_slice_count
                .ok_or(crate::Error {
                    missing_field: "resource_slice_count",
                })?,
        })
    }
    fn merge(&mut self, other: ResourcePoolAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.generation {
            self.generation = other_value;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.resource_slice_count {
            self.resource_slice_count = other_value;
        }
        Ok(())
    }
}
