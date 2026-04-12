#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// EndpointSubset is a group of addresses with a common set of ports. The expanded set of endpoints is the Cartesian product of Addresses x Ports. For example, given:
///
///   {
///       Addresses: \[{"ip": "10.10.1.1"}, {"ip": "10.10.2.2"}\],
///       Ports:     \[{"name": "a", "port": 8675}, {"name": "b", "port": 309}\]
///     }
///
/// The resulting set of endpoints can be viewed as:
///
///   a: \[ 10.10.1.1:8675, 10.10.2.2:8675 \],
///     b: \[ 10.10.1.1:309, 10.10.2.2:309 \]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EndpointSubsetAc {
    /// IP addresses which offer the related ports that are marked as ready. These endpoints should be considered safe for load balancers and clients to utilize.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::EndpointAddress as crate::Optionable>::Optioned,
        >,
    >,
    /// IP addresses which offer the related ports but are not currently marked as ready because they have not yet finished starting, have recently failed a readiness check, or have recently failed a liveness check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_ready_addresses: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::EndpointAddress as crate::Optionable>::Optioned,
        >,
    >,
    /// Port numbers available on the related IP addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::core::v1::EndpointPort as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::EndpointSubset {
    type Optioned = EndpointSubsetAc;
}
#[automatically_derived]
impl crate::Optionable for EndpointSubsetAc {
    type Optioned = EndpointSubsetAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::EndpointSubset {
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
        if self.addresses.is_none() {
            self.addresses = other.addresses;
        }
        if let Some(other_value) = other.addresses {
            self.addresses = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.not_ready_addresses.is_none() {
            self.not_ready_addresses = other.not_ready_addresses;
        }
        if let Some(other_value) = other.not_ready_addresses {
            self.not_ready_addresses = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if self.ports.is_none() {
            self.ports = other.ports;
        }
        if let Some(other_value) = other.ports {
            self.ports = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::EndpointSubset>
for EndpointSubsetAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::EndpointSubset) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::EndpointSubset, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::EndpointSubset,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
