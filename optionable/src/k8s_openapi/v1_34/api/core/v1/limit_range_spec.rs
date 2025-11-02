#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
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
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LimitRangeSpec {
    fn into_optioned(self) -> LimitRangeSpecAc {
        LimitRangeSpecAc {
            limits: Some(crate::OptionableConvert::into_optioned(self.limits)),
        }
    }
    fn try_from_optioned(
        value: LimitRangeSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            limits: crate::OptionableConvert::try_from_optioned(
                value
                    .limits
                    .ok_or(crate::optionable::Error {
                        missing_field: "limits",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: LimitRangeSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.limits {
            crate::OptionableConvert::merge(&mut self.limits, other_value)?;
        }
        Ok(())
    }
}
