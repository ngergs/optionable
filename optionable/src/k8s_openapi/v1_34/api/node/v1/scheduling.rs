pub struct SchedulingOpt {
    pub node_selector: <Option<
        std::collections::BTreeMap<std::string::String, std::string::String>,
    > as crate::Optionable>::Optioned,
    pub tolerations: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::Toleration>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::node::v1::Scheduling {
    type Optioned = SchedulingOpt;
}
#[automatically_derived]
impl crate::Optionable for SchedulingOpt {
    type Optioned = SchedulingOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::node::v1::Scheduling {
    fn into_optioned(self) -> SchedulingOpt {
        SchedulingOpt {
            node_selector: crate::OptionableConvert::into_optioned(self.node_selector),
            tolerations: crate::OptionableConvert::into_optioned(self.tolerations),
        }
    }
    fn try_from_optioned(
        value: SchedulingOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            node_selector: crate::OptionableConvert::try_from_optioned(
                value.node_selector,
            )?,
            tolerations: crate::OptionableConvert::try_from_optioned(value.tolerations)?,
        })
    }
    fn merge(&mut self, other: SchedulingOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.node_selector, other.node_selector)?;
        crate::OptionableConvert::merge(&mut self.tolerations, other.tolerations)?;
        Ok(())
    }
}
