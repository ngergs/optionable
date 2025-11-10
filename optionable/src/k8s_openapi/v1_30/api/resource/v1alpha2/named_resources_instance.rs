#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct NamedResourcesInstanceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: <Option<
        std::vec::Vec<::k8s_openapi::api::resource::v1alpha2::NamedResourcesAttribute>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesInstance {
    type Optioned = NamedResourcesInstanceAc;
}
#[automatically_derived]
impl crate::Optionable for NamedResourcesInstanceAc {
    type Optioned = NamedResourcesInstanceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesInstance {
    fn into_optioned(self) -> NamedResourcesInstanceAc {
        NamedResourcesInstanceAc {
            attributes: crate::OptionableConvert::into_optioned(self.attributes),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(value: NamedResourcesInstanceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            attributes: crate::OptionableConvert::try_from_optioned(value.attributes)?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NamedResourcesInstanceAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.attributes, other.attributes)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
