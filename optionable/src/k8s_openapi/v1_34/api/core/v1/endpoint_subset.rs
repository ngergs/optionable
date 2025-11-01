pub struct EndpointSubsetAc {
    pub addresses: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::EndpointAddress>,
    > as crate::Optionable>::Optioned,
    pub not_ready_addresses: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::EndpointAddress>,
    > as crate::Optionable>::Optioned,
    pub ports: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::EndpointPort>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EndpointSubset {
    type Optioned = EndpointSubsetAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointSubsetAc {
    type Optioned = EndpointSubsetAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EndpointSubset {
    fn into_optioned(self) -> EndpointSubsetAc {
        EndpointSubsetAc {
            addresses: crate::OptionableConvert::into_optioned(self.addresses),
            not_ready_addresses: crate::OptionableConvert::into_optioned(
                self.not_ready_addresses,
            ),
            ports: crate::OptionableConvert::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: EndpointSubsetAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            addresses: crate::OptionableConvert::try_from_optioned(value.addresses)?,
            not_ready_addresses: crate::OptionableConvert::try_from_optioned(
                value.not_ready_addresses,
            )?,
            ports: crate::OptionableConvert::try_from_optioned(value.ports)?,
        })
    }
    fn merge(
        &mut self,
        other: EndpointSubsetAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.addresses, other.addresses)?;
        crate::OptionableConvert::merge(
            &mut self.not_ready_addresses,
            other.not_ready_addresses,
        )?;
        crate::OptionableConvert::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
