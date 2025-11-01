pub struct NodeSwapStatusAc {
    pub capacity: <Option<i64> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::NodeSwapStatus {
    type Optioned = NodeSwapStatusAc;
}
#[automatically_derived]
impl crate::Optionable for NodeSwapStatusAc {
    type Optioned = NodeSwapStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::NodeSwapStatus {
    fn into_optioned(self) -> NodeSwapStatusAc {
        NodeSwapStatusAc {
            capacity: crate::OptionableConvert::into_optioned(self.capacity),
        }
    }
    fn try_from_optioned(
        value: NodeSwapStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            capacity: crate::OptionableConvert::try_from_optioned(value.capacity)?,
        })
    }
    fn merge(
        &mut self,
        other: NodeSwapStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.capacity, other.capacity)?;
        Ok(())
    }
}
