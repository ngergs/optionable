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
        if self.accepted_names.is_none() {
            self.accepted_names = crate::OptionableConvert::try_from_optioned(
                other.accepted_names,
            )?;
        } else if let Some(self_value) = self.accepted_names.as_mut()
            && let Some(other_value) = other.accepted_names
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.conditions.is_none() {
            self.conditions = crate::OptionableConvert::try_from_optioned(
                other.conditions,
            )?;
        } else if let Some(self_value) = self.conditions.as_mut()
            && let Some(other_value) = other.conditions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.stored_versions.is_none() {
            self.stored_versions = crate::OptionableConvert::try_from_optioned(
                other.stored_versions,
            )?;
        } else if let Some(self_value) = self.stored_versions.as_mut()
            && let Some(other_value) = other.stored_versions
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
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
impl k8s_openapi027::DeepMerge for CustomResourceDefinitionStatusAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.accepted_names,
            other.accepted_names,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.conditions, other.conditions);
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.stored_versions,
            other.stored_versions,
        );
    }
}
