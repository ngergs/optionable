pub struct CSINodeSpecOpt {
    pub drivers: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::storage::v1::CSINodeDriver,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::CSINodeSpec {
    type Optioned = CSINodeSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for CSINodeSpecOpt {
    type Optioned = CSINodeSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::CSINodeSpec {
    fn into_optioned(self) -> CSINodeSpecOpt {
        CSINodeSpecOpt {
            drivers: Some(
                <std::vec::Vec<
                    ::k8s_openapi::api::storage::v1::CSINodeDriver,
                > as crate::OptionableConvert>::into_optioned(self.drivers),
            ),
        }
    }
    fn try_from_optioned(
        value: CSINodeSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            drivers: <std::vec::Vec<
                ::k8s_openapi::api::storage::v1::CSINodeDriver,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .drivers
                    .ok_or(crate::optionable::Error {
                        missing_field: "drivers",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CSINodeSpecOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.drivers {
            <std::vec::Vec<
                ::k8s_openapi::api::storage::v1::CSINodeDriver,
            > as crate::OptionableConvert>::merge(&mut self.drivers, other_value)?;
        }
        Ok(())
    }
}
