#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceConstraint must have exactly one field set besides Requests.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceConstraintAc {
    /// DistinctAttribute requires that all devices in question have this attribute and that its type and value are unique across those devices.
    ///
    /// This acts as the inverse of MatchAttribute.
    ///
    /// This constraint is used to avoid allocating multiple requests to the same device by ensuring attribute-level differentiation.
    ///
    /// This is useful for scenarios where resource requests must be fulfilled by separate physical devices. For example, a container requests two network interfaces that must be allocated from two different physical NICs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_attribute: Option<std::string::String>,
    /// MatchAttribute requires that all devices in question have this attribute and that its type and value are the same across those devices.
    ///
    /// For example, if you specified "dra.example.com/numa" (a hypothetical example!), then only devices in the same NUMA node will be chosen. A device which does not have that attribute will not be chosen. All devices should use a value of the same type for this attribute because that is part of its specification, but if one device doesn't, then it also will not be chosen.
    ///
    /// Must include the domain qualifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_attribute: Option<std::string::String>,
    /// Requests is a list of the one or more requests in this claim which must co-satisfy this constraint. If a request is fulfilled by multiple devices, then all of the devices must satisfy the constraint. If this is not specified, this constraint applies to all requests in this claim.
    ///
    /// References to subrequests must include the name of the main request and may include the subrequest using the format \<main request\>\[/\<subrequest\>\]. If just the main request is given, the constraint applies to all subrequests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta1::DeviceConstraint {
    type Optioned = DeviceConstraintAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceConstraintAc {
    type Optioned = DeviceConstraintAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1beta1::DeviceConstraint {
    fn into_optioned(self) -> DeviceConstraintAc {
        DeviceConstraintAc {
            distinct_attribute: self.distinct_attribute,
            match_attribute: self.match_attribute,
            requests: self.requests,
        }
    }
    fn try_from_optioned(value: DeviceConstraintAc) -> Result<Self, crate::Error> {
        Ok(Self {
            distinct_attribute: value.distinct_attribute,
            match_attribute: value.match_attribute,
            requests: value.requests,
        })
    }
    fn merge(&mut self, other: DeviceConstraintAc) -> Result<(), crate::Error> {
        if self.distinct_attribute.is_none() {
            self.distinct_attribute = other.distinct_attribute;
        }
        if let Some(other_value) = other.distinct_attribute {
            crate::OptionableConvert::merge(&mut self.distinct_attribute, other_value)?;
        }
        if self.match_attribute.is_none() {
            self.match_attribute = other.match_attribute;
        }
        if let Some(other_value) = other.match_attribute {
            crate::OptionableConvert::merge(&mut self.match_attribute, other_value)?;
        }
        if self.requests.is_none() {
            self.requests = other.requests;
        }
        if let Some(other_value) = other.requests {
            self.requests = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta1::DeviceConstraint>
for DeviceConstraintAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta1::DeviceConstraint,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta1::DeviceConstraint, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta1::DeviceConstraint,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
