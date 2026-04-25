#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceClaim defines how to request devices with a ResourceClaim.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceClaimAc {
    /// This field holds configuration for multiple potential drivers which could satisfy requests in this claim. It is ignored while allocating the claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta2::DeviceClaimConfiguration as crate::Optionable>::Optioned,
        >,
    >,
    /// These constraints must be satisfied by the set of devices that get allocated for the claim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta2::DeviceConstraint as crate::Optionable>::Optioned,
        >,
    >,
    /// Requests represent individual requests for distinct devices which must all be satisfied. If empty, nothing needs to be allocated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<
        std::vec::Vec<
            <::k8s_openapi027::api::resource::v1beta2::DeviceRequest as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta2::DeviceClaim {
    type Optioned = DeviceClaimAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceClaimAc {
    type Optioned = DeviceClaimAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1beta2::DeviceClaim {
    fn into_optioned(self) -> DeviceClaimAc {
        DeviceClaimAc {
            config: crate::OptionableConvert::into_optioned(self.config),
            constraints: crate::OptionableConvert::into_optioned(self.constraints),
            requests: crate::OptionableConvert::into_optioned(self.requests),
        }
    }
    fn try_from_optioned(value: DeviceClaimAc) -> Result<Self, crate::Error> {
        Ok(Self {
            config: crate::OptionableConvert::try_from_optioned(value.config)?,
            constraints: crate::OptionableConvert::try_from_optioned(value.constraints)?,
            requests: crate::OptionableConvert::try_from_optioned(value.requests)?,
        })
    }
    fn merge(&mut self, other: DeviceClaimAc) -> Result<(), crate::Error> {
        if self.config.is_none() {
            self.config = crate::OptionableConvert::try_from_optioned(other.config)?;
        } else if let Some(self_value) = self.config.as_mut()
            && let Some(other_value) = other.config
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.constraints.is_none() {
            self.constraints = crate::OptionableConvert::try_from_optioned(
                other.constraints,
            )?;
        } else if let Some(self_value) = self.constraints.as_mut()
            && let Some(other_value) = other.constraints
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        if self.requests.is_none() {
            self.requests = crate::OptionableConvert::try_from_optioned(other.requests)?;
        } else if let Some(self_value) = self.requests.as_mut()
            && let Some(other_value) = other.requests
        {
            *self_value = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta2::DeviceClaim>
for DeviceClaimAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::DeviceClaim,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta2::DeviceClaim, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::DeviceClaim,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for DeviceClaimAc {
    fn merge_from(&mut self, other: Self) {
        self.config = other.config;
        self.constraints = other.constraints;
        self.requests = other.requests;
    }
}
