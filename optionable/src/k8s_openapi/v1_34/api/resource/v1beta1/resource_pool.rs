pub struct ResourcePoolOpt {
    pub generation: Option<i64>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub resource_slice_count: Option<i64>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1beta1::ResourcePool {
    type Optioned = ResourcePoolOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourcePoolOpt {
    type Optioned = ResourcePoolOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::resource::v1beta1::ResourcePool {
    fn into_optioned(self) -> ResourcePoolOpt {
        ResourcePoolOpt {
            generation: Some(self.generation),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            resource_slice_count: Some(self.resource_slice_count),
        }
    }
    fn try_from_optioned(
        value: ResourcePoolOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            generation: value
                .generation
                .ok_or(crate::optionable::Error {
                    missing_field: "generation",
                })?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            resource_slice_count: value
                .resource_slice_count
                .ok_or(crate::optionable::Error {
                    missing_field: "resource_slice_count",
                })?,
        })
    }
    fn merge(&mut self, other: ResourcePoolOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.generation {
            self.generation = other_value;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.resource_slice_count {
            self.resource_slice_count = other_value;
        }
        Ok(())
    }
}
