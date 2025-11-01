pub struct LimitRangeOpt {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub spec: <Option<
        ::k8s_openapi::api::core::v1::LimitRangeSpec,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::LimitRange {
    type Optioned = LimitRangeOpt;
}
#[automatically_derived]
impl crate::Optionable for LimitRangeOpt {
    type Optioned = LimitRangeOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::LimitRange {
    fn into_optioned(self) -> LimitRangeOpt {
        LimitRangeOpt {
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
        }
    }
    fn try_from_optioned(
        value: LimitRangeOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
        })
    }
    fn merge(&mut self, other: LimitRangeOpt) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        Ok(())
    }
}
