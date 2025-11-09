#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NamedResourcesFilterAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesFilter {
    type Optioned = NamedResourcesFilterAc;
}
#[automatically_derived]
impl crate::Optionable for NamedResourcesFilterAc {
    type Optioned = NamedResourcesFilterAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesFilter {
    fn into_optioned(self) -> NamedResourcesFilterAc {
        NamedResourcesFilterAc {
            selector: Some(crate::OptionableConvert::into_optioned(self.selector)),
        }
    }
    fn try_from_optioned(value: NamedResourcesFilterAc) -> Result<Self, crate::Error> {
        Ok(Self {
            selector: crate::OptionableConvert::try_from_optioned(
                value
                    .selector
                    .ok_or(crate::Error {
                        missing_field: "selector",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NamedResourcesFilterAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.selector {
            crate::OptionableConvert::merge(&mut self.selector, other_value)?;
        }
        Ok(())
    }
}
