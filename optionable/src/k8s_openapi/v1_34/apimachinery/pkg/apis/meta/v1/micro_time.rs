#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MicroTimeAc(
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Option<
        <::k8s_openapi::chrono::DateTime<
            ::k8s_openapi::chrono::Utc,
        > as crate::Optionable>::Optioned,
    >,
);
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime {
    type Optioned = MicroTimeAc;
}
#[automatically_derived]
impl crate::Optionable for MicroTimeAc {
    type Optioned = MicroTimeAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime {
    fn into_optioned(self) -> MicroTimeAc {
        MicroTimeAc(Some(crate::OptionableConvert::into_optioned(self.0)))
    }
    fn try_from_optioned(value: MicroTimeAc) -> Result<Self, crate::optionable::Error> {
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
    fn merge(&mut self, other: MicroTimeAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.0 {
            crate::OptionableConvert::merge(&mut self.0, other_value)?;
        }
        Ok(())
    }
}
