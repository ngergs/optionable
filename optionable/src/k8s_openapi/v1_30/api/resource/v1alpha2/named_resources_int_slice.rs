#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha2::NamedResourcesIntSlice {
    fn into_optioned(self) -> NamedResourcesIntSliceAc {
        NamedResourcesIntSliceAc {
            ints: Some(crate::OptionableConvert::into_optioned(self.ints)),
        }
    }
    fn try_from_optioned(value: NamedResourcesIntSliceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            ints: crate::OptionableConvert::try_from_optioned(
                value
                    .ints
                    .ok_or(crate::Error {
                        missing_field: "ints",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: NamedResourcesIntSliceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.ints {
            crate::OptionableConvert::merge(&mut self.ints, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::resource::v1alpha2::NamedResourcesIntSlice,
> for NamedResourcesIntSliceAc {
    fn from_optionable(
        value: ::k8s_openapi::api::resource::v1alpha2::NamedResourcesIntSlice,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::resource::v1alpha2::NamedResourcesIntSlice,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::resource::v1alpha2::NamedResourcesIntSlice,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
