#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceFieldSelectorAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divisor: <Option<
        ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceFieldSelector {
    type Optioned = ResourceFieldSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceFieldSelectorAc {
    type Optioned = ResourceFieldSelectorAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceFieldSelector {
    fn into_optioned(self) -> ResourceFieldSelectorAc {
        ResourceFieldSelectorAc {
            container_name: crate::OptionableConvert::into_optioned(self.container_name),
            divisor: crate::OptionableConvert::into_optioned(self.divisor),
            resource: Some(crate::OptionableConvert::into_optioned(self.resource)),
        }
    }
    fn try_from_optioned(
        value: ResourceFieldSelectorAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_name: crate::OptionableConvert::try_from_optioned(
                value.container_name,
            )?,
            divisor: crate::OptionableConvert::try_from_optioned(value.divisor)?,
            resource: crate::OptionableConvert::try_from_optioned(
                value
                    .resource
                    .ok_or(crate::optionable::Error {
                        missing_field: "resource",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceFieldSelectorAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.container_name, other.container_name)?;
        crate::OptionableConvert::merge(&mut self.divisor, other.divisor)?;
        if let Some(other_value) = other.resource {
            crate::OptionableConvert::merge(&mut self.resource, other_value)?;
        }
        Ok(())
    }
}
