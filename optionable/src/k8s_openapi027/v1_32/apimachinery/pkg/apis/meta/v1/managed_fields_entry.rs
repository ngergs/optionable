#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ManagedFieldsEntryAc {
    /// APIVersion defines the version of this resource that this field set applies to. The format is "group/version" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<std::string::String>,
    /// FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: "FieldsV1"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_type: Option<std::string::String>,
    /// FieldsV1 holds the first JSON version format as described in the "FieldsV1" type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_v1: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldsV1 as crate::Optionable>::Optioned,
    >,
    /// Manager is an identifier of the workflow managing these fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<std::string::String>,
    /// Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<std::string::String>,
    /// Subresource is the name of the subresource used to update that object, or empty string if the object was updated through the main resource. The value of this field is used to distinguish between managers, even if they share the same name. For example, a status update will be distinct from a regular update using the same manager name. Note that the APIVersion field is not related to the Subresource field and it always corresponds to the version of the main resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource: Option<std::string::String>,
    /// Time is the timestamp of when the ManagedFields entry was added. The timestamp will also be updated if a field is added, the manager changes any of the owned fields value or removes a field. The timestamp does not update when a field is removed from the entry because another manager took it over.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry {
    type Optioned = ManagedFieldsEntryAc;
}
#[automatically_derived]
impl crate::Optionable for ManagedFieldsEntryAc {
    type Optioned = ManagedFieldsEntryAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry {
    fn into_optioned(self) -> ManagedFieldsEntryAc {
        ManagedFieldsEntryAc {
            api_version: self.api_version,
            fields_type: self.fields_type,
            fields_v1: crate::OptionableConvert::into_optioned(self.fields_v1),
            manager: self.manager,
            operation: self.operation,
            subresource: self.subresource,
            time: crate::OptionableConvert::into_optioned(self.time),
        }
    }
    fn try_from_optioned(value: ManagedFieldsEntryAc) -> Result<Self, crate::Error> {
        Ok(Self {
            api_version: value.api_version,
            fields_type: value.fields_type,
            fields_v1: crate::OptionableConvert::try_from_optioned(value.fields_v1)?,
            manager: value.manager,
            operation: value.operation,
            subresource: value.subresource,
            time: crate::OptionableConvert::try_from_optioned(value.time)?,
        })
    }
    fn merge(&mut self, other: ManagedFieldsEntryAc) -> Result<(), crate::Error> {
        if other.api_version.is_some() {
            self.api_version = other.api_version;
        }
        if other.fields_type.is_some() {
            self.fields_type = other.fields_type;
        }
        crate::OptionableConvert::merge(&mut self.fields_v1, other.fields_v1)?;
        if other.manager.is_some() {
            self.manager = other.manager;
        }
        if other.operation.is_some() {
            self.operation = other.operation;
        }
        if other.subresource.is_some() {
            self.subresource = other.subresource;
        }
        crate::OptionableConvert::merge(&mut self.time, other.time)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
> for ManagedFieldsEntryAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
