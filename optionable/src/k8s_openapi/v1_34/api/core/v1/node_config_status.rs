pub struct NodeConfigStatusOpt {
    pub active: <Option<
        ::k8s_openapi::api::core::v1::NodeConfigSource,
    > as crate::Optionable>::Optioned,
    pub assigned: <Option<
        ::k8s_openapi::api::core::v1::NodeConfigSource,
    > as crate::Optionable>::Optioned,
    pub error: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub last_known_good: <Option<
        ::k8s_openapi::api::core::v1::NodeConfigSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeConfigStatus {
    type Optioned = NodeConfigStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeConfigStatusOpt {
    type Optioned = NodeConfigStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeConfigStatus {
    fn into_optioned(self) -> NodeConfigStatusOpt {
        NodeConfigStatusOpt {
            active: crate::OptionableConvert::into_optioned(self.active),
            assigned: crate::OptionableConvert::into_optioned(self.assigned),
            error: crate::OptionableConvert::into_optioned(self.error),
            last_known_good: crate::OptionableConvert::into_optioned(
                self.last_known_good,
            ),
        }
    }
    fn try_from_optioned(
        value: NodeConfigStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            active: crate::OptionableConvert::try_from_optioned(value.active)?,
            assigned: crate::OptionableConvert::try_from_optioned(value.assigned)?,
            error: crate::OptionableConvert::try_from_optioned(value.error)?,
            last_known_good: crate::OptionableConvert::try_from_optioned(
                value.last_known_good,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: NodeConfigStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.active, other.active)?;
        crate::OptionableConvert::merge(&mut self.assigned, other.assigned)?;
        crate::OptionableConvert::merge(&mut self.error, other.error)?;
        crate::OptionableConvert::merge(
            &mut self.last_known_good,
            other.last_known_good,
        )?;
        Ok(())
    }
}
