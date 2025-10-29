pub struct EndpointHintsOpt {
    pub for_nodes: <Option<
        std::vec::Vec<::k8s_openapi::api::discovery::v1::ForNode>,
    > as crate::Optionable>::Optioned,
    pub for_zones: <Option<
        std::vec::Vec<::k8s_openapi::api::discovery::v1::ForZone>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::discovery::v1::EndpointHints {
    type Optioned = EndpointHintsOpt;
}
#[automatically_derived]
impl crate::Optionable for EndpointHintsOpt {
    type Optioned = EndpointHintsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::discovery::v1::EndpointHints {
    fn into_optioned(self) -> EndpointHintsOpt {
        EndpointHintsOpt {
            for_nodes: crate::OptionableConvert::into_optioned(self.for_nodes),
            for_zones: crate::OptionableConvert::into_optioned(self.for_zones),
        }
    }
    fn try_from_optioned(
        value: EndpointHintsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            for_nodes: crate::OptionableConvert::try_from_optioned(value.for_nodes)?,
            for_zones: crate::OptionableConvert::try_from_optioned(value.for_zones)?,
        })
    }
    fn merge(
        &mut self,
        other: EndpointHintsOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.for_nodes, other.for_nodes)?;
        crate::OptionableConvert::merge(&mut self.for_zones, other.for_zones)?;
        Ok(())
    }
}
