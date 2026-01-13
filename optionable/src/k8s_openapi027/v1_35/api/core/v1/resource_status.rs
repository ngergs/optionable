#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: <Option<
        std::vec::Vec<::k8s_openapi027::api::core::v1::ResourceHealth>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ResourceStatus {
    type Optioned = ResourceStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceStatusAc {
    type Optioned = ResourceStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ResourceStatus {
    fn into_optioned(self) -> ResourceStatusAc {
        ResourceStatusAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            resources: crate::OptionableConvert::into_optioned(self.resources),
        }
    }
    fn try_from_optioned(value: ResourceStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            resources: crate::OptionableConvert::try_from_optioned(value.resources)?,
        })
    }
    fn merge(&mut self, other: ResourceStatusAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.resources, other.resources)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ResourceStatus>
for ResourceStatusAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ResourceStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ResourceStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ResourceStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
