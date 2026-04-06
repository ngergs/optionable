#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectMetaAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_grace_period_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalizers: Option<std::vec::Vec<std::string::String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_fields: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_references: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::OwnerReference as crate::Optionable>::Optioned,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta {
    type Optioned = ObjectMetaAc;
}
#[automatically_derived]
impl crate::Optionable for ObjectMetaAc {
    type Optioned = ObjectMetaAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta {
    fn into_optioned(self) -> ObjectMetaAc {
        ObjectMetaAc {
            annotations: self.annotations,
            creation_timestamp: crate::OptionableConvert::into_optioned(
                self.creation_timestamp,
            ),
            deletion_grace_period_seconds: self.deletion_grace_period_seconds,
            deletion_timestamp: crate::OptionableConvert::into_optioned(
                self.deletion_timestamp,
            ),
            finalizers: self.finalizers,
            generate_name: self.generate_name,
            generation: self.generation,
            labels: self.labels,
            managed_fields: crate::OptionableConvert::into_optioned(self.managed_fields),
            name: self.name,
            namespace: self.namespace,
            owner_references: crate::OptionableConvert::into_optioned(
                self.owner_references,
            ),
            resource_version: self.resource_version,
            self_link: self.self_link,
            uid: self.uid,
        }
    }
    fn try_from_optioned(value: ObjectMetaAc) -> Result<Self, crate::Error> {
        Ok(Self {
            annotations: value.annotations,
            creation_timestamp: crate::OptionableConvert::try_from_optioned(
                value.creation_timestamp,
            )?,
            deletion_grace_period_seconds: value.deletion_grace_period_seconds,
            deletion_timestamp: crate::OptionableConvert::try_from_optioned(
                value.deletion_timestamp,
            )?,
            finalizers: value.finalizers,
            generate_name: value.generate_name,
            generation: value.generation,
            labels: value.labels,
            managed_fields: crate::OptionableConvert::try_from_optioned(
                value.managed_fields,
            )?,
            name: value.name,
            namespace: value.namespace,
            owner_references: crate::OptionableConvert::try_from_optioned(
                value.owner_references,
            )?,
            resource_version: value.resource_version,
            self_link: value.self_link,
            uid: value.uid,
        })
    }
    fn merge(&mut self, other: ObjectMetaAc) -> Result<(), crate::Error> {
        self.annotations = other.annotations;
        crate::OptionableConvert::merge(
            &mut self.creation_timestamp,
            other.creation_timestamp,
        )?;
        self.deletion_grace_period_seconds = other.deletion_grace_period_seconds;
        crate::OptionableConvert::merge(
            &mut self.deletion_timestamp,
            other.deletion_timestamp,
        )?;
        self.finalizers = other.finalizers;
        self.generate_name = other.generate_name;
        self.generation = other.generation;
        self.labels = other.labels;
        crate::OptionableConvert::merge(&mut self.managed_fields, other.managed_fields)?;
        self.name = other.name;
        self.namespace = other.namespace;
        crate::OptionableConvert::merge(
            &mut self.owner_references,
            other.owner_references,
        )?;
        self.resource_version = other.resource_version;
        self.self_link = other.self_link;
        self.uid = other.uid;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
> for ObjectMetaAc {
    fn from_optionable(
        value: k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
