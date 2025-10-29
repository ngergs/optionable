pub struct MicroTimeOpt(
    pub Option<
        <::k8s_openapi::chrono::DateTime<
            ::k8s_openapi::chrono::Utc,
        > as crate::Optionable>::Optioned,
    >,
);
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime {
    type Optioned = MicroTimeOpt;
}
#[automatically_derived]
impl crate::Optionable for MicroTimeOpt {
    type Optioned = MicroTimeOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime {
    fn into_optioned(self) -> MicroTimeOpt {
        MicroTimeOpt(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: MicroTimeOpt) -> Result<Self, crate::optionable::Error> {
        Ok(
            Self(
                crate::OptionableConvert::try_from_optioned(
                    value
                        .0
                        .ok_or(crate::optionable::Error {
                            missing_field: "0",
                        })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: MicroTimeOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
