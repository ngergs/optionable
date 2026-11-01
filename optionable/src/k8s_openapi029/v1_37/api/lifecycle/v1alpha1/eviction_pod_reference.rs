#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EvictionPodReference contains enough information to locate the referenced pod inside the same namespace.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvictionPodReferenceAc {
    /// name of the target. This field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    /// uid of the target. It can be found in .metadata.uid of the target and is a lowercase UUID in 8-4-4-4-12 format. This field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi029::api::lifecycle::v1alpha1::EvictionPodReference {
    type Optioned = EvictionPodReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for EvictionPodReferenceAc {
    type Optioned = EvictionPodReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::lifecycle::v1alpha1::EvictionPodReference {
    fn into_optioned(self) -> EvictionPodReferenceAc {
        EvictionPodReferenceAc {
            name: Some(self.name),
            uid: Some(self.uid),
        }
    }
    fn try_from_optioned(value: EvictionPodReferenceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
            uid: value
                .uid
                .ok_or(crate::Error {
                    missing_field: "uid",
                })?,
        })
    }
    fn merge(&mut self, other: EvictionPodReferenceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.uid {
            self.uid = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi029::api::lifecycle::v1alpha1::EvictionPodReference,
> for EvictionPodReferenceAc {
    fn from_optionable(
        value: k8s_openapi029::api::lifecycle::v1alpha1::EvictionPodReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi029::api::lifecycle::v1alpha1::EvictionPodReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::lifecycle::v1alpha1::EvictionPodReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for EvictionPodReferenceAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.name, other.name);
        k8s_openapi029::DeepMerge::merge_from(&mut self.uid, other.uid);
    }
}
