#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EndpointHints provides hints describing how an endpoint should be consumed.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointHintsAc {
    /// forZones indicates the zone(s) this endpoint should be consumed by to enable topology aware routing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_zones: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::discovery::v1::ForZone as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::discovery::v1::EndpointHints {
    type Optioned = EndpointHintsAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointHintsAc {
    type Optioned = EndpointHintsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::discovery::v1::EndpointHints {
    fn into_optioned(self) -> EndpointHintsAc {
        EndpointHintsAc {
            for_zones: crate::OptionableConvert::into_optioned(self.for_zones),
        }
    }
    fn try_from_optioned(value: EndpointHintsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            for_zones: crate::OptionableConvert::try_from_optioned(value.for_zones)?,
        })
    }
    fn merge(&mut self, other: EndpointHintsAc) -> Result<(), crate::Error> {
        if self.for_zones.is_none() {
            self.for_zones = crate::OptionableConvert::try_from_optioned(
                other.for_zones,
            )?;
        } else if let Some(self_value) = self.for_zones.as_mut()
            && let Some(other_value) = other.for_zones
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::discovery::v1::EndpointHints>
for EndpointHintsAc {
    fn from_optionable(
        value: k8s_openapi027::api::discovery::v1::EndpointHints,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::discovery::v1::EndpointHints, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::discovery::v1::EndpointHints,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for EndpointHintsAc {
    fn merge_from(&mut self, other: Self) {
        self.for_zones = other.for_zones;
    }
}
