#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct NodeRuntimeHandlerFeaturesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_read_only_mounts: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_namespaces: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeRuntimeHandlerFeatures {
    type Optioned = NodeRuntimeHandlerFeaturesAc;
}
#[automatically_derived]
impl crate::Optionable for NodeRuntimeHandlerFeaturesAc {
    type Optioned = NodeRuntimeHandlerFeaturesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::NodeRuntimeHandlerFeatures {
    fn into_optioned(self) -> NodeRuntimeHandlerFeaturesAc {
        NodeRuntimeHandlerFeaturesAc {
            recursive_read_only_mounts: crate::OptionableConvert::into_optioned(
                self.recursive_read_only_mounts,
            ),
            user_namespaces: crate::OptionableConvert::into_optioned(
                self.user_namespaces,
            ),
        }
    }
    fn try_from_optioned(
        value: NodeRuntimeHandlerFeaturesAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            recursive_read_only_mounts: crate::OptionableConvert::try_from_optioned(
                value.recursive_read_only_mounts,
            )?,
            user_namespaces: crate::OptionableConvert::try_from_optioned(
                value.user_namespaces,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NodeRuntimeHandlerFeaturesAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.recursive_read_only_mounts,
            other.recursive_read_only_mounts,
        )?;
        crate::OptionableConvert::merge(
            &mut self.user_namespaces,
            other.user_namespaces,
        )?;
        Ok(())
    }
}
