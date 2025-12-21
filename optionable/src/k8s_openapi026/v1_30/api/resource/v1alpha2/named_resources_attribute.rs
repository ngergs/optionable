#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NamedResourcesAttributeAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bool: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_slice: <Option<
        ::k8s_openapi026::api::resource::v1alpha2::NamedResourcesIntSlice,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: <Option<
        ::k8s_openapi026::apimachinery::pkg::api::resource::Quantity,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_slice: <Option<
        ::k8s_openapi026::api::resource::v1alpha2::NamedResourcesStringSlice,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::resource::v1alpha2::NamedResourcesAttribute {
    type Optioned = NamedResourcesAttributeAc;
}
#[automatically_derived]
impl crate::Optionable for NamedResourcesAttributeAc {
    type Optioned = NamedResourcesAttributeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::resource::v1alpha2::NamedResourcesAttribute {
    fn into_optioned(self) -> NamedResourcesAttributeAc {
        NamedResourcesAttributeAc {
            bool: crate::OptionableConvert::into_optioned(self.bool),
            int: crate::OptionableConvert::into_optioned(self.int),
            int_slice: crate::OptionableConvert::into_optioned(self.int_slice),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            quantity: crate::OptionableConvert::into_optioned(self.quantity),
            string: crate::OptionableConvert::into_optioned(self.string),
            string_slice: crate::OptionableConvert::into_optioned(self.string_slice),
            version: crate::OptionableConvert::into_optioned(self.version),
        }
    }
    fn try_from_optioned(
        value: NamedResourcesAttributeAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            bool: crate::OptionableConvert::try_from_optioned(value.bool)?,
            int: crate::OptionableConvert::try_from_optioned(value.int)?,
            int_slice: crate::OptionableConvert::try_from_optioned(value.int_slice)?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            quantity: crate::OptionableConvert::try_from_optioned(value.quantity)?,
            string: crate::OptionableConvert::try_from_optioned(value.string)?,
            string_slice: crate::OptionableConvert::try_from_optioned(
                value.string_slice,
            )?,
            version: crate::OptionableConvert::try_from_optioned(value.version)?,
        })
    }
    fn merge(&mut self, other: NamedResourcesAttributeAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.bool, other.bool)?;
        crate::OptionableConvert::merge(&mut self.int, other.int)?;
        crate::OptionableConvert::merge(&mut self.int_slice, other.int_slice)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.quantity, other.quantity)?;
        crate::OptionableConvert::merge(&mut self.string, other.string)?;
        crate::OptionableConvert::merge(&mut self.string_slice, other.string_slice)?;
        crate::OptionableConvert::merge(&mut self.version, other.version)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::resource::v1alpha2::NamedResourcesAttribute,
> for NamedResourcesAttributeAc {
    fn from_optionable(
        value: k8s_openapi026::api::resource::v1alpha2::NamedResourcesAttribute,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::resource::v1alpha2::NamedResourcesAttribute,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::resource::v1alpha2::NamedResourcesAttribute,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
