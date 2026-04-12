#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceDefinitionStatusAc {
    /// acceptedNames are the names that are actually being used to serve discovery. They may be different than the names in spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_names: Option<
        <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionNames as crate::Optionable>::Optioned,
    >,
    /// conditions indicate state for particular aspects of a CustomResourceDefinition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<
        std::vec::Vec<
            <::k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionCondition as crate::Optionable>::Optioned,
        >,
    >,
    /// storedVersions lists all versions of CustomResources that were ever persisted. Tracking these versions allows a migration path for stored versions in etcd. The field is mutable so a migration controller can finish a migration to another version (ensuring no old objects are left in storage), and then remove the rest of the versions from this list. Versions may not be removed from `spec.versions` while they exist in this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_versions: Option<std::vec::Vec<std::string::String>>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionStatus {
    type Optioned = CustomResourceDefinitionStatusAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceDefinitionStatusAc {
    type Optioned = CustomResourceDefinitionStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionStatus {
    fn into_optioned(self) -> CustomResourceDefinitionStatusAc {
        CustomResourceDefinitionStatusAc {
            accepted_names: crate::OptionableConvert::into_optioned(self.accepted_names),
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
            stored_versions: self.stored_versions,
        }
    }
    fn try_from_optioned(
        value: CustomResourceDefinitionStatusAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            accepted_names: crate::OptionableConvert::try_from_optioned(
                value.accepted_names,
            )?,
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
            stored_versions: value.stored_versions,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceDefinitionStatusAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.accepted_names, other.accepted_names)?;
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        if other.stored_versions.is_some() {
            self.stored_versions = other.stored_versions;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionStatus,
> for CustomResourceDefinitionStatusAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinitionStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
