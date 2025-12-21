#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceHealthAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(rename = "resourceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::ResourceHealth {
    type Optioned = ResourceHealthAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceHealthAc {
    type Optioned = ResourceHealthAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::ResourceHealth {
    fn into_optioned(self) -> ResourceHealthAc {
        ResourceHealthAc {
            health: crate::OptionableConvert::into_optioned(self.health),
            resource_id: Some(crate::OptionableConvert::into_optioned(self.resource_id)),
        }
    }
    fn try_from_optioned(value: ResourceHealthAc) -> Result<Self, crate::Error> {
        Ok(Self {
            health: crate::OptionableConvert::try_from_optioned(value.health)?,
            resource_id: crate::OptionableConvert::try_from_optioned(
                value
                    .resource_id
                    .ok_or(crate::Error {
                        missing_field: "resource_id",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ResourceHealthAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.health, other.health)?;
        if let Some(other_value) = other.resource_id {
            crate::OptionableConvert::merge(&mut self.resource_id, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::ResourceHealth>
for ResourceHealthAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::ResourceHealth) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::ResourceHealth, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::ResourceHealth,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
