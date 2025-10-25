pub struct TimeOpt(
    pub Option<
        <::k8s_openapi::chrono::DateTime<
            ::k8s_openapi::chrono::Utc,
        > as crate::Optionable>::Optioned,
    >,
);
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time {
    type Optioned = TimeOpt;
}
#[automatically_derived]
impl crate::Optionable for TimeOpt {
    type Optioned = TimeOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time {
    fn into_optioned(self) -> TimeOpt {
        TimeOpt(
            Some(
                <::k8s_openapi::chrono::DateTime<
                    ::k8s_openapi::chrono::Utc,
                > as crate::OptionableConvert>::into_optioned(self.0),
            ),
        )
    }
    fn try_from_optioned(value: TimeOpt) -> Result<Self, crate::optionable::Error> {
        Ok(
            Self(
                <::k8s_openapi::chrono::DateTime<
                    ::k8s_openapi::chrono::Utc,
                > as crate::OptionableConvert>::try_from_optioned(
                    value
                        .0
                        .ok_or(crate::optionable::Error {
                            missing_field: "0",
                        })?,
                )?,
            ),
        )
    }
    fn merge(&mut self, other: TimeOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            <::k8s_openapi::chrono::DateTime<
                ::k8s_openapi::chrono::Utc,
            > as crate::OptionableConvert>::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
