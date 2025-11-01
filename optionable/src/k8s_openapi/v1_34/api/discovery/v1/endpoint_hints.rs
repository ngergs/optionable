pub struct EndpointHintsAc {
    pub for_nodes: <Option<
        std::vec::Vec<::k8s_openapi::api::discovery::v1::ForNode>,
    > as crate::Optionable>::Optioned,
    pub for_zones: <Option<
        std::vec::Vec<::k8s_openapi::api::discovery::v1::ForZone>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::discovery::v1::EndpointHints {
    type Optioned = EndpointHintsAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointHintsAc {
    type Optioned = EndpointHintsAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::discovery::v1::EndpointHints {
    fn into_optioned(self) -> EndpointHintsAc {
        EndpointHintsAc {
            for_nodes: crate::OptionableConvert::into_optioned(self.for_nodes),
            for_zones: crate::OptionableConvert::into_optioned(self.for_zones),
        }
    }
    fn try_from_optioned(
        value: EndpointHintsAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            for_nodes: crate::OptionableConvert::try_from_optioned(value.for_nodes)?,
            for_zones: crate::OptionableConvert::try_from_optioned(value.for_zones)?,
        })
    }
    fn merge(&mut self, other: EndpointHintsAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.for_nodes, other.for_nodes)?;
        crate::OptionableConvert::merge(&mut self.for_zones, other.for_zones)?;
        Ok(())
    }
}
