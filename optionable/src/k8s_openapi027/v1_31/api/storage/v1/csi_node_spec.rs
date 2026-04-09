#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CSINodeSpec holds information about the specification of all CSI drivers installed on a node
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CSINodeSpecAc {
    /// drivers is a list of information of all CSI Drivers existing on a node. If all drivers in the list are uninstalled, this can become empty.
    pub drivers: std::vec::Vec<::k8s_openapi027::api::storage::v1::CSINodeDriver>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::storage::v1::CSINodeSpec {
    type Optioned = CSINodeSpecAc;
}
#[automatically_derived]
impl crate::Optionable for CSINodeSpecAc {
    type Optioned = CSINodeSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::storage::v1::CSINodeSpec {
    fn into_optioned(self) -> CSINodeSpecAc {
        CSINodeSpecAc {
            drivers: self.drivers,
        }
    }
    fn try_from_optioned(value: CSINodeSpecAc) -> Result<Self, crate::Error> {
        Ok(Self { drivers: value.drivers })
    }
    fn merge(&mut self, other: CSINodeSpecAc) -> Result<(), crate::Error> {
        self.drivers = other.drivers;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::storage::v1::CSINodeSpec>
for CSINodeSpecAc {
    fn from_optionable(value: k8s_openapi027::api::storage::v1::CSINodeSpec) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::storage::v1::CSINodeSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::storage::v1::CSINodeSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
