#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ResourceHealth represents the health of a resource. It has the latest device health information. This is a part of KEP https://kep.k8s.io/4680 and historical health changes are planned to be added in future iterations of a KEP.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceHealthAc {
    /// Health of the resource. can be one of:
    ///  - Healthy: operates as normal
    ///  - Unhealthy: reported unhealthy. We consider this a temporary health issue
    ///               since we do not have a mechanism today to distinguish
    ///               temporary and permanent issues.
    ///  - Unknown: The status cannot be determined.
    ///             For example, Device Plugin got unregistered and hasn't been re-registered since.
    ///
    /// In future we may want to introduce the PermanentlyUnhealthy Status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<std::string::String>,
    /// ResourceID is the unique identifier of the resource. See the ResourceID type for more information.
    #[serde(rename = "resourceID")]
    pub resource_id: std::string::String,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ResourceHealth {
    type Optioned = ResourceHealthAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceHealthAc {
    type Optioned = ResourceHealthAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ResourceHealth {
    fn into_optioned(self) -> ResourceHealthAc {
        ResourceHealthAc {
            health: self.health,
            resource_id: self.resource_id,
        }
    }
    fn try_from_optioned(value: ResourceHealthAc) -> Result<Self, crate::Error> {
        Ok(Self {
            health: value.health,
            resource_id: value.resource_id,
        })
    }
    fn merge(&mut self, other: ResourceHealthAc) -> Result<(), crate::Error> {
        self.health = other.health;
        self.resource_id = other.resource_id;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ResourceHealth>
for ResourceHealthAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ResourceHealth) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ResourceHealth, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ResourceHealth,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
