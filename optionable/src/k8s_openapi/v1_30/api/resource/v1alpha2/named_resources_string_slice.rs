#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct NamedResourcesStringSliceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strings: Option<
        <std::vec::Vec<std::string::String> as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesStringSlice {
    type Optioned = NamedResourcesStringSliceAc;
}
#[automatically_derived]
impl crate::Optionable for NamedResourcesStringSliceAc {
    type Optioned = NamedResourcesStringSliceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesStringSlice {
    fn into_optioned(self) -> NamedResourcesStringSliceAc {
        NamedResourcesStringSliceAc {
            strings: Some(crate::OptionableConvert::into_optioned(self.strings)),
        }
    }
    fn try_from_optioned(
        value: NamedResourcesStringSliceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            strings: crate::OptionableConvert::try_from_optioned(
                value
                    .strings
                    .ok_or(crate::Error {
                        missing_field: "strings",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NamedResourcesStringSliceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.strings {
            crate::OptionableConvert::merge(&mut self.strings, other_value)?;
        }
        Ok(())
    }
}
