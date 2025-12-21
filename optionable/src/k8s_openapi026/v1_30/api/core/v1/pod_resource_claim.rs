#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodResourceClaimAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: <Option<
        ::k8s_openapi026::api::core::v1::ClaimSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::PodResourceClaim {
    type Optioned = PodResourceClaimAc;
}
#[automatically_derived]
impl crate::Optionable for PodResourceClaimAc {
    type Optioned = PodResourceClaimAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::PodResourceClaim {
    fn into_optioned(self) -> PodResourceClaimAc {
        PodResourceClaimAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            source: crate::OptionableConvert::into_optioned(self.source),
        }
    }
    fn try_from_optioned(value: PodResourceClaimAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            source: crate::OptionableConvert::try_from_optioned(value.source)?,
        })
    }
    fn merge(&mut self, other: PodResourceClaimAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.source, other.source)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::PodResourceClaim>
for PodResourceClaimAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::PodResourceClaim) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::PodResourceClaim, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::PodResourceClaim,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
