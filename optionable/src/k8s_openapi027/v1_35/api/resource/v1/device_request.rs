#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceRequest is a request for devices required for a claim. This is typically a request for a single resource like a device, but can also ask for several identical devices. With FirstAvailable it is also possible to provide a prioritized list of requests.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceRequestAc {
    /// Exactly specifies the details for a single request that must be met exactly for the request to be satisfied.
    ///
    /// One of Exactly or FirstAvailable must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exactly: Option<
        <::k8s_openapi027::api::resource::v1::ExactDeviceRequest as crate::Optionable>::Optioned,
    >,
    /// FirstAvailable contains subrequests, of which exactly one will be selected by the scheduler. It tries to satisfy them in the order in which they are listed here. So if there are two entries in the list, the scheduler will only check the second one if it determines that the first one can not be used.
    ///
    /// DRA does not yet implement scoring, so the scheduler will select the first set of devices that satisfies all the requests in the claim. And if the requirements can be satisfied on more than one node, other scheduling features will determine which node is chosen. This means that the set of devices allocated to a claim might not be the optimal set available to the cluster. Scoring will be implemented later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_available: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1::DeviceSubRequest as crate::Optionable>::Optioned,
        >,
    >,
    /// Name can be used to reference this request in a pod.spec.containers\[\].resources.claims entry and in a constraint of the claim.
    ///
    /// References using the name in the DeviceRequest will uniquely identify a request when the Exactly field is set. When the FirstAvailable field is set, a reference to the name of the DeviceRequest will match whatever subrequest is chosen by the scheduler.
    ///
    /// Must be a DNS label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1::DeviceRequest {
    type Optioned = DeviceRequestAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceRequestAc {
    type Optioned = DeviceRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1::DeviceRequest {
    fn into_optioned(self) -> DeviceRequestAc {
        DeviceRequestAc {
            exactly: crate::OptionableConvert::into_optioned(self.exactly),
            first_available: crate::OptionableConvert::into_optioned(
                self.first_available,
            ),
            name: Some(self.name),
        }
    }
    fn try_from_optioned(value: DeviceRequestAc) -> Result<Self, crate::Error> {
        Ok(Self {
            exactly: crate::OptionableConvert::try_from_optioned(value.exactly)?,
            first_available: crate::OptionableConvert::try_from_optioned(
                value.first_available,
            )?,
            name: value
                .name
                .ok_or(crate::Error {
                    missing_field: "name",
                })?,
        })
    }
    fn merge(&mut self, other: DeviceRequestAc) -> Result<(), crate::Error> {
        if self.exactly.is_none() {
            self.exactly = crate::OptionableConvert::try_from_optioned(other.exactly)?;
        } else if let Some(self_value) = self.exactly.as_mut()
            && let Some(other_value) = other.exactly
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.first_available.is_none() {
            self.first_available = crate::OptionableConvert::try_from_optioned(
                other.first_available,
            )?;
        } else if let Some(self_value) = self.first_available.as_mut()
            && let Some(other_value) = other.first_available
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if let Some(other_value) = other.name {
            self.name = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1::DeviceRequest>
for DeviceRequestAc {
    fn from_optionable(value: k8s_openapi027::api::resource::v1::DeviceRequest) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1::DeviceRequest, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1::DeviceRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for DeviceRequestAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(&mut self.exactly, other.exactly);
        self.first_available = other.first_available;
        k8s_openapi027::DeepMerge::merge_from(&mut self.name, other.name);
    }
}
