pub struct LimitRangeSpecOpt {
    pub limits: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::core::v1::LimitRangeItem,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::LimitRangeSpec {
    type Optioned = LimitRangeSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for LimitRangeSpecOpt {
    type Optioned = LimitRangeSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LimitRangeSpec {
    fn into_optioned(self) -> LimitRangeSpecOpt {
        LimitRangeSpecOpt {
            limits: Some(crate::OptionableConvert::into_optioned(self.limits)),
        }
    }
    fn try_from_optioned(
        value: LimitRangeSpecOpt,
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
        other: LimitRangeSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.limits {
            crate::OptionableConvert::merge(&mut self.limits, other_value)?;
        }
        Ok(())
    }
}
