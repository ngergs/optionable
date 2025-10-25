pub struct NodeRuntimeHandlerFeaturesOpt {
    pub recursive_read_only_mounts: <Option<bool> as crate::Optionable>::Optioned,
    pub user_namespaces: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeRuntimeHandlerFeatures {
    type Optioned = NodeRuntimeHandlerFeaturesOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeRuntimeHandlerFeaturesOpt {
    type Optioned = NodeRuntimeHandlerFeaturesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::NodeRuntimeHandlerFeatures {
    fn into_optioned(self) -> NodeRuntimeHandlerFeaturesOpt {
        NodeRuntimeHandlerFeaturesOpt {
            recursive_read_only_mounts: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(
                self.recursive_read_only_mounts,
            ),
            user_namespaces: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.user_namespaces),
        }
    }
    fn try_from_optioned(
        value: NodeRuntimeHandlerFeaturesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            recursive_read_only_mounts: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.recursive_read_only_mounts,
            )?,
            user_namespaces: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.user_namespaces)?,
        })
    }
    fn merge(
        &mut self,
        other: NodeRuntimeHandlerFeaturesOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.recursive_read_only_mounts,
            other.recursive_read_only_mounts,
        )?;
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.user_namespaces,
            other.user_namespaces,
        )?;
        Ok(())
    }
}
