#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointSubsetAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::EndpointAddress>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_ready_addresses: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::EndpointAddress>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi026::api::core::v1::EndpointPort>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::EndpointSubset {
    type Optioned = EndpointSubsetAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointSubsetAc {
    type Optioned = EndpointSubsetAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::EndpointSubset {
    fn into_optioned(self) -> EndpointSubsetAc {
        EndpointSubsetAc {
            addresses: crate::OptionableConvert::into_optioned(self.addresses),
            not_ready_addresses: crate::OptionableConvert::into_optioned(
                self.not_ready_addresses,
            ),
            ports: crate::OptionableConvert::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(value: EndpointSubsetAc) -> Result<Self, crate::Error> {
        Ok(Self {
            addresses: crate::OptionableConvert::try_from_optioned(value.addresses)?,
            not_ready_addresses: crate::OptionableConvert::try_from_optioned(
                value.not_ready_addresses,
            )?,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
        })
    }
    fn merge(&mut self, other: EndpointSubsetAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.addresses, other.addresses)?;
        crate::OptionableConvert::merge(
            &mut self.not_ready_addresses,
            other.not_ready_addresses,
        )?;
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::EndpointSubset>
for EndpointSubsetAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::EndpointSubset) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::EndpointSubset, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::EndpointSubset,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
