#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::CSINodeSpec {
    fn into_optioned(self) -> CSINodeSpecAc {
        CSINodeSpecAc {
            drivers: Some(crate::OptionableConvert::into_optioned(self.drivers)),
        }
    }
    fn try_from_optioned(value: CSINodeSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            drivers: crate::OptionableConvert::try_from_optioned(
                value
                    .drivers
                    .ok_or(crate::Error {
                        missing_field: "drivers",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CSINodeSpecAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.drivers {
            crate::OptionableConvert::merge(&mut self.drivers, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::storage::v1::CSINodeSpec>
for CSINodeSpecAc {
    fn from_optionable(value: ::k8s_openapi::api::storage::v1::CSINodeSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::storage::v1::CSINodeSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::storage::v1::CSINodeSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
