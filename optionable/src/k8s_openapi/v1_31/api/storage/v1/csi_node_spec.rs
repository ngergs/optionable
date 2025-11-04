#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct CSINodeSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::storage::v1::CSINodeDriver,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::CSINodeSpec {
    type Optioned = CSINodeSpecAc;
}
#[automatically_derived]
impl crate::Optionable for CSINodeSpecAc {
    type Optioned = CSINodeSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::CSINodeSpec {
    fn into_optioned(self) -> CSINodeSpecAc {
        CSINodeSpecAc {
            drivers: Some(crate::OptionableConvert::into_optioned(self.drivers)),
        }
    }
    fn try_from_optioned(
        value: CSINodeSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            drivers: crate::OptionableConvert::try_from_optioned(
                value
                    .drivers
                    .ok_or(crate::optionable::Error {
                        missing_field: "drivers",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CSINodeSpecAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.drivers {
            crate::OptionableConvert::merge(&mut self.drivers, other_value)?;
        }
        Ok(())
    }
}
