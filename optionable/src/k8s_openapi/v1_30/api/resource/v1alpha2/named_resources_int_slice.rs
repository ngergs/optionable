#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NamedResourcesIntSliceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ints: Option<<std::vec::Vec<i64> as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesIntSlice {
    type Optioned = NamedResourcesIntSliceAc;
}
#[automatically_derived]
impl crate::Optionable for NamedResourcesIntSliceAc {
    type Optioned = NamedResourcesIntSliceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesIntSlice {
    fn into_optioned(self) -> NamedResourcesIntSliceAc {
        NamedResourcesIntSliceAc {
            ints: Some(crate::OptionableConvert::into_optioned(self.ints)),
        }
    }
    fn try_from_optioned(
        value: NamedResourcesIntSliceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ints: crate::OptionableConvert::try_from_optioned(
                value
                    .ints
                    .ok_or(crate::optionable::Error {
                        missing_field: "ints",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NamedResourcesIntSliceAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.ints {
            crate::OptionableConvert::merge(&mut self.ints, other_value)?;
        }
        Ok(())
    }
}
