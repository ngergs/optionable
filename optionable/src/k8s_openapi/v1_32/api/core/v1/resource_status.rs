#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::ResourceHealth>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceStatus {
    type Optioned = ResourceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceStatusAc {
    type Optioned = ResourceStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceStatus {
    fn into_optioned(self) -> ResourceStatusAc {
        ResourceStatusAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            resources: crate::OptionableConvert::into_optioned(self.resources),
        }
    }
    fn try_from_optioned(
        value: ResourceStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        Ok(())
    }
}
