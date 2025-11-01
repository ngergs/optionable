#[derive(kube::Resource)]
#[resource(inherit = StorageClass)]
pub struct StorageClassAc {
    pub allow_volume_expansion: <Option<bool> as crate::Optionable>::Optioned,
    pub allowed_topologies: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::TopologySelectorTerm>,
    > as crate::Optionable>::Optioned,
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    pub mount_options: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub parameters: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub provisioner: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub reclaim_policy: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub volume_binding_mode: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::StorageClass {
    type Optioned = StorageClassAc;
}
#[automatically_derived]
impl crate::Optionable for StorageClassAc {
    type Optioned = StorageClassAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::StorageClass {
    fn into_optioned(self) -> StorageClassAc {
        StorageClassAc {
            allow_volume_expansion: crate::OptionableConvert::into_optioned(
                self.allow_volume_expansion,
            ),
            allowed_topologies: crate::OptionableConvert::into_optioned(
                self.allowed_topologies,
            ),
            metadata: self.metadata,
            mount_options: crate::OptionableConvert::into_optioned(self.mount_options),
            parameters: crate::OptionableConvert::into_optioned(self.parameters),
            provisioner: Some(crate::OptionableConvert::into_optioned(self.provisioner)),
            reclaim_policy: crate::OptionableConvert::into_optioned(self.reclaim_policy),
            volume_binding_mode: crate::OptionableConvert::into_optioned(
                self.volume_binding_mode,
            ),
        }
    }
    fn try_from_optioned(
        value: StorageClassAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allow_volume_expansion: crate::OptionableConvert::try_from_optioned(
                value.allow_volume_expansion,
            )?,
            allowed_topologies: crate::OptionableConvert::try_from_optioned(
                value.allowed_topologies,
            )?,
            metadata: value.metadata,
            mount_options: crate::OptionableConvert::try_from_optioned(
                value.mount_options,
            )?,
            parameters: crate::OptionableConvert::try_from_optioned(value.parameters)?,
            provisioner: crate::OptionableConvert::try_from_optioned(
                value
                    .provisioner
                    .ok_or(crate::optionable::Error {
                        missing_field: "provisioner",
                    })?,
            )?,
            reclaim_policy: crate::OptionableConvert::try_from_optioned(
                value.reclaim_policy,
            )?,
            volume_binding_mode: crate::OptionableConvert::try_from_optioned(
                value.volume_binding_mode,
            )?,
        })
    }
    fn merge(&mut self, other: StorageClassAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.allow_volume_expansion,
            other.allow_volume_expansion,
        )?;
        crate::OptionableConvert::merge(
            &mut self.allowed_topologies,
            other.allowed_topologies,
        )?;
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.mount_options, other.mount_options)?;
        crate::OptionableConvert::merge(&mut self.parameters, other.parameters)?;
        if let Some(other_value) = other.provisioner {
            crate::OptionableConvert::merge(&mut self.provisioner, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.reclaim_policy, other.reclaim_policy)?;
        crate::OptionableConvert::merge(
            &mut self.volume_binding_mode,
            other.volume_binding_mode,
        )?;
        Ok(())
    }
}
#[allow(unused_imports)]
use ::k8s_openapi::api::storage::v1::StorageClass;
