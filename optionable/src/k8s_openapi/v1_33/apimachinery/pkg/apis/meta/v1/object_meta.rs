#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ObjectMetaAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_grace_period_seconds: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalizers: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_fields: <Option<
        std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_references: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
    type Optioned = ObjectMetaAc;
}
#[automatically_derived]
impl crate::Optionable for ObjectMetaAc {
    type Optioned = ObjectMetaAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
    fn into_optioned(self) -> ObjectMetaAc {
        ObjectMetaAc {
            annotations: crate::OptionableConvert::into_optioned(self.annotations),
            creation_timestamp: crate::OptionableConvert::into_optioned(
                self.creation_timestamp,
            ),
            deletion_grace_period_seconds: crate::OptionableConvert::into_optioned(
                self.deletion_grace_period_seconds,
            ),
            deletion_timestamp: crate::OptionableConvert::into_optioned(
                self.deletion_timestamp,
            ),
            finalizers: crate::OptionableConvert::into_optioned(self.finalizers),
            generate_name: crate::OptionableConvert::into_optioned(self.generate_name),
            generation: crate::OptionableConvert::into_optioned(self.generation),
            labels: crate::OptionableConvert::into_optioned(self.labels),
            managed_fields: crate::OptionableConvert::into_optioned(self.managed_fields),
            name: crate::OptionableConvert::into_optioned(self.name),
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
            owner_references: crate::OptionableConvert::into_optioned(
                self.owner_references,
            ),
            resource_version: crate::OptionableConvert::into_optioned(
                self.resource_version,
            ),
            self_link: crate::OptionableConvert::into_optioned(self.self_link),
            uid: crate::OptionableConvert::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(value: ObjectMetaAc) -> Result<Self, crate::Error> {
        Ok(Self {
            annotations: crate::OptionableConvert::try_from_optioned(value.annotations)?,
            creation_timestamp: crate::OptionableConvert::try_from_optioned(
                value.creation_timestamp,
            )?,
            deletion_grace_period_seconds: crate::OptionableConvert::try_from_optioned(
                value.deletion_grace_period_seconds,
            )?,
            deletion_timestamp: crate::OptionableConvert::try_from_optioned(
                value.deletion_timestamp,
            )?,
            finalizers: crate::OptionableConvert::try_from_optioned(value.finalizers)?,
            generate_name: crate::OptionableConvert::try_from_optioned(
                value.generate_name,
            )?,
            generation: crate::OptionableConvert::try_from_optioned(value.generation)?,
            labels: crate::OptionableConvert::try_from_optioned(value.labels)?,
            managed_fields: crate::OptionableConvert::try_from_optioned(
                value.managed_fields,
            )?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
            owner_references: crate::OptionableConvert::try_from_optioned(
                value.owner_references,
            )?,
            resource_version: crate::OptionableConvert::try_from_optioned(
                value.resource_version,
            )?,
            self_link: crate::OptionableConvert::try_from_optioned(value.self_link)?,
            uid: crate::OptionableConvert::try_from_optioned(value.uid)?,
        })
    }
    fn merge(&mut self, other: ObjectMetaAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.annotations, other.annotations)?;
        crate::OptionableConvert::merge(
            &mut self.creation_timestamp,
            other.creation_timestamp,
        )?;
        crate::OptionableConvert::merge(
            &mut self.deletion_grace_period_seconds,
            other.deletion_grace_period_seconds,
        )?;
        crate::OptionableConvert::merge(
            &mut self.deletion_timestamp,
            other.deletion_timestamp,
        )?;
        crate::OptionableConvert::merge(&mut self.finalizers, other.finalizers)?;
        crate::OptionableConvert::merge(&mut self.generate_name, other.generate_name)?;
        crate::OptionableConvert::merge(&mut self.generation, other.generation)?;
        crate::OptionableConvert::merge(&mut self.labels, other.labels)?;
        crate::OptionableConvert::merge(&mut self.managed_fields, other.managed_fields)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        crate::OptionableConvert::merge(
            &mut self.owner_references,
            other.owner_references,
        )?;
        crate::OptionableConvert::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        crate::OptionableConvert::merge(&mut self.self_link, other.self_link)?;
        crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
