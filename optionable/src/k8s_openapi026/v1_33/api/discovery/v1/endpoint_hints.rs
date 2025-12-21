#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointHintsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_nodes: <Option<
        std::vec::Vec<::k8s_openapi026::api::discovery::v1::ForNode>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_zones: <Option<
        std::vec::Vec<::k8s_openapi026::api::discovery::v1::ForZone>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::discovery::v1::EndpointHints {
    type Optioned = EndpointHintsAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointHintsAc {
    type Optioned = EndpointHintsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::discovery::v1::EndpointHints {
    fn into_optioned(self) -> EndpointHintsAc {
        EndpointHintsAc {
            for_nodes: crate::OptionableConvert::into_optioned(self.for_nodes),
            for_zones: crate::OptionableConvert::into_optioned(self.for_zones),
        }
    }
    fn try_from_optioned(value: EndpointHintsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            for_nodes: crate::OptionableConvert::try_from_optioned(value.for_nodes)?,
            for_zones: crate::OptionableConvert::try_from_optioned(value.for_zones)?,
        })
    }
    fn merge(&mut self, other: EndpointHintsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.for_nodes, other.for_nodes)?;
        crate::OptionableConvert::merge(&mut self.for_zones, other.for_zones)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::discovery::v1::EndpointHints>
for EndpointHintsAc {
    fn from_optionable(
        value: k8s_openapi026::api::discovery::v1::EndpointHints,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::discovery::v1::EndpointHints, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::discovery::v1::EndpointHints,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
