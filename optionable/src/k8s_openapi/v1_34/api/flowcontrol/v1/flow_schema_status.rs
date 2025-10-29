pub struct FlowSchemaStatusOpt {
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::FlowSchemaCondition>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::FlowSchemaStatus {
    type Optioned = FlowSchemaStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaStatusOpt {
    type Optioned = FlowSchemaStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::flowcontrol::v1::FlowSchemaStatus {
    fn into_optioned(self) -> FlowSchemaStatusOpt {
        FlowSchemaStatusOpt {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: FlowSchemaStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: FlowSchemaStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
