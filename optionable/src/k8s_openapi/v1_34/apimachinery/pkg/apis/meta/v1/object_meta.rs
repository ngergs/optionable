pub struct ObjectMetaOpt {
    pub annotations: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub creation_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub deletion_grace_period_seconds: <Option<i64> as crate::Optionable>::Optioned,
    pub deletion_timestamp: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    pub finalizers: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub generate_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub generation: <Option<i64> as crate::Optionable>::Optioned,
    pub labels: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub managed_fields: <Option<
        std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
        >,
    > as crate::Optionable>::Optioned,
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub owner_references: <Option<
        std::vec::Vec<::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference>,
    > as crate::Optionable>::Optioned,
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub self_link: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
    type Optioned = ObjectMetaOpt;
}
#[automatically_derived]
impl crate::Optionable for ObjectMetaOpt {
    type Optioned = ObjectMetaOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
    fn into_optioned(self) -> ObjectMetaOpt {
        ObjectMetaOpt {
            annotations: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.annotations),
            creation_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.creation_timestamp),
            deletion_grace_period_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(
                self.deletion_grace_period_seconds,
            ),
            deletion_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::into_optioned(self.deletion_timestamp),
            finalizers: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.finalizers),
            generate_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.generate_name),
            generation: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.generation),
            labels: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::into_optioned(self.labels),
            managed_fields: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
                >,
            > as crate::OptionableConvert>::into_optioned(self.managed_fields),
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.name),
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.namespace),
            owner_references: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference,
                >,
            > as crate::OptionableConvert>::into_optioned(self.owner_references),
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.resource_version),
            self_link: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.self_link),
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(
        value: ObjectMetaOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            annotations: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.annotations)?,
            creation_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.creation_timestamp)?,
            deletion_grace_period_seconds: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.deletion_grace_period_seconds,
            )?,
            deletion_timestamp: <Option<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
            > as crate::OptionableConvert>::try_from_optioned(value.deletion_timestamp)?,
            finalizers: <Option<
                std::vec::Vec<std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.finalizers)?,
            generate_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.generate_name)?,
            generation: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.generation)?,
            labels: <Option<
                std::collections::BTreeMap<std::string::String, std::string::String>,
            > as crate::OptionableConvert>::try_from_optioned(value.labels)?,
            managed_fields: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.managed_fields)?,
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.name)?,
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.namespace)?,
            owner_references: <Option<
                std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference,
                >,
            > as crate::OptionableConvert>::try_from_optioned(value.owner_references)?,
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_version)?,
            self_link: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.self_link)?,
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.uid)?,
        })
    }
    fn merge(&mut self, other: ObjectMetaOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.annotations, other.annotations)?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.creation_timestamp,
            other.creation_timestamp,
        )?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.deletion_grace_period_seconds,
            other.deletion_grace_period_seconds,
        )?;
        <Option<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Time,
        > as crate::OptionableConvert>::merge(
            &mut self.deletion_timestamp,
            other.deletion_timestamp,
        )?;
        <Option<
            std::vec::Vec<std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.finalizers, other.finalizers)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.generate_name,
            other.generate_name,
        )?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(&mut self.generation, other.generation)?;
        <Option<
            std::collections::BTreeMap<std::string::String, std::string::String>,
        > as crate::OptionableConvert>::merge(&mut self.labels, other.labels)?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.managed_fields,
            other.managed_fields,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.name, other.name)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.namespace, other.namespace)?;
        <Option<
            std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::OwnerReference,
            >,
        > as crate::OptionableConvert>::merge(
            &mut self.owner_references,
            other.owner_references,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.self_link, other.self_link)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
