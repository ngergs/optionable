pub struct EndpointSubsetOpt {
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
impl crate::Optionable
for ::k8s_openapi::api::core::v1::endpoint_subset::EndpointSubset {
    type Optioned = EndpointSubsetOpt;
}
#[automatically_derived]
impl crate::Optionable for EndpointSubsetOpt {
    type Optioned = EndpointSubsetOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::endpoint_subset::EndpointSubset {
    fn into_optioned(self) -> EndpointSubsetOpt {
        EndpointSubsetOpt {
            addresses: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EndpointAddress>,
            > as crate::OptionableConvert>::into_optioned(self.addresses),
            not_ready_addresses: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EndpointAddress>,
            > as crate::OptionableConvert>::into_optioned(self.not_ready_addresses),
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EndpointPort>,
            > as crate::OptionableConvert>::into_optioned(self.ports),
        }
    }
    fn try_from_optioned(
        value: EndpointSubsetOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            addresses: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EndpointAddress>,
            > as crate::OptionableConvert>::try_from_optioned(value.addresses)?,
            not_ready_addresses: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EndpointAddress>,
            > as crate::OptionableConvert>::try_from_optioned(
                value.not_ready_addresses,
            )?,
            ports: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::EndpointPort>,
            > as crate::OptionableConvert>::try_from_optioned(value.ports)?,
        })
    }
    fn merge(
        &mut self,
        other: EndpointSubsetOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::EndpointAddress>,
        > as crate::OptionableConvert>::merge(&mut self.addresses, other.addresses)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::EndpointAddress>,
        > as crate::OptionableConvert>::merge(
            &mut self.not_ready_addresses,
            other.not_ready_addresses,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::EndpointPort>,
        > as crate::OptionableConvert>::merge(&mut self.ports, other.ports)?;
        Ok(())
    }
}
