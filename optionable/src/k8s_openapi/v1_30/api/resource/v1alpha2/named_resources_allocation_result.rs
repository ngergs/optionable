#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NamedResourcesAllocationResultAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesAllocationResult {
    type Optioned = NamedResourcesAllocationResultAc;
}
#[automatically_derived]
impl crate::Optionable for NamedResourcesAllocationResultAc {
    type Optioned = NamedResourcesAllocationResultAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesAllocationResult {
    fn into_optioned(self) -> NamedResourcesAllocationResultAc {
        NamedResourcesAllocationResultAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: NamedResourcesAllocationResultAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NamedResourcesAllocationResultAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
