#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct LimitRangeSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::core::v1::LimitRangeItem,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::LimitRangeSpec {
    type Optioned = LimitRangeSpecAc;
}
#[automatically_derived]
impl crate::Optionable for LimitRangeSpecAc {
    type Optioned = LimitRangeSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LimitRangeSpec {
    fn into_optioned(self) -> LimitRangeSpecAc {
        LimitRangeSpecAc {
            limits: Some(crate::OptionableConvert::into_optioned(self.limits)),
        }
    }
    fn try_from_optioned(value: LimitRangeSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            limits: crate::OptionableConvert::try_from_optioned(
                value
                    .limits
                    .ok_or(crate::Error {
                        missing_field: "limits",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: LimitRangeSpecAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.limits {
            crate::OptionableConvert::merge(&mut self.limits, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::LimitRangeSpec>
for LimitRangeSpecAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::LimitRangeSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::LimitRangeSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::LimitRangeSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
