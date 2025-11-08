#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimSchedulingStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuitable_nodes: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha3::ResourceClaimSchedulingStatus {
    type Optioned = ResourceClaimSchedulingStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimSchedulingStatusAc {
    type Optioned = ResourceClaimSchedulingStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::ResourceClaimSchedulingStatus {
    fn into_optioned(self) -> ResourceClaimSchedulingStatusAc {
        ResourceClaimSchedulingStatusAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            unsuitable_nodes: crate::OptionableConvert::into_optioned(
                self.unsuitable_nodes,
            ),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimSchedulingStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            unsuitable_nodes: crate::OptionableConvert::try_from_optioned(
                value.unsuitable_nodes,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimSchedulingStatusAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.unsuitable_nodes,
            other.unsuitable_nodes,
        )?;
        Ok(())
    }
}
