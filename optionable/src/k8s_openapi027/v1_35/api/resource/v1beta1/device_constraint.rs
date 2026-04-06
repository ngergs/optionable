#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceConstraintAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct_attribute: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_attribute: Option<std::string::String>,
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
        self.distinct_attribute = other.distinct_attribute;
        self.match_attribute = other.match_attribute;
        self.requests = other.requests;
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
