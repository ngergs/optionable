pub struct NodeSwapStatusOpt {
    pub capacity: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::node_swap_status::NodeSwapStatus {
    type Optioned = NodeSwapStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for NodeSwapStatusOpt {
    type Optioned = NodeSwapStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::node_swap_status::NodeSwapStatus {
    fn into_optioned(self) -> NodeSwapStatusOpt {
        NodeSwapStatusOpt {
            capacity: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.capacity),
        }
    }
    fn try_from_optioned(
        value: NodeSwapStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            capacity: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(value.capacity)?,
        })
    }
    fn merge(
        &mut self,
        other: NodeSwapStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(&mut self.capacity, other.capacity)?;
        Ok(())
    }
}
