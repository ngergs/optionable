#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NodeRuntimeHandlerFeatures is a set of features implemented by the runtime handler.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeRuntimeHandlerFeaturesAc {
    /// RecursiveReadOnlyMounts is set to true if the runtime handler supports RecursiveReadOnlyMounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_read_only_mounts: Option<bool>,
    /// UserNamespaces is set to true if the runtime handler supports UserNamespaces, including for volumes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_namespaces: Option<bool>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::NodeRuntimeHandlerFeatures {
    type Optioned = NodeRuntimeHandlerFeaturesAc;
}
#[automatically_derived]
impl crate::Optionable for NodeRuntimeHandlerFeaturesAc {
    type Optioned = NodeRuntimeHandlerFeaturesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::NodeRuntimeHandlerFeatures {
    fn into_optioned(self) -> NodeRuntimeHandlerFeaturesAc {
        NodeRuntimeHandlerFeaturesAc {
            recursive_read_only_mounts: self.recursive_read_only_mounts,
            user_namespaces: self.user_namespaces,
        }
    }
    fn try_from_optioned(
        value: NodeRuntimeHandlerFeaturesAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            recursive_read_only_mounts: value.recursive_read_only_mounts,
            user_namespaces: value.user_namespaces,
        })
    }
    fn merge(
        &mut self,
        other: NodeRuntimeHandlerFeaturesAc,
    ) -> Result<(), crate::Error> {
        if self.recursive_read_only_mounts.is_none() {
            self.recursive_read_only_mounts = crate::OptionableConvert::try_from_optioned(
                other.recursive_read_only_mounts,
            )?;
        } else if let Some(self_value) = self.recursive_read_only_mounts.as_mut()
            && let Some(other_value) = other.recursive_read_only_mounts
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.user_namespaces.is_none() {
            self.user_namespaces = crate::OptionableConvert::try_from_optioned(
                other.user_namespaces,
            )?;
        } else if let Some(self_value) = self.user_namespaces.as_mut()
            && let Some(other_value) = other.user_namespaces
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::NodeRuntimeHandlerFeatures>
for NodeRuntimeHandlerFeaturesAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::NodeRuntimeHandlerFeatures,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::NodeRuntimeHandlerFeatures,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::NodeRuntimeHandlerFeatures,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for NodeRuntimeHandlerFeaturesAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.recursive_read_only_mounts,
            other.recursive_read_only_mounts,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.user_namespaces,
            other.user_namespaces,
        );
    }
}
