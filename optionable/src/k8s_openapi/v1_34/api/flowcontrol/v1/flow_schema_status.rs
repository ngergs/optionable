#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSchemaStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::flowcontrol::v1::FlowSchemaCondition>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::FlowSchemaStatus {
    type Optioned = FlowSchemaStatusAc;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaStatusAc {
    type Optioned = FlowSchemaStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::flowcontrol::v1::FlowSchemaStatus {
    fn into_optioned(self) -> FlowSchemaStatusAc {
        FlowSchemaStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(
        value: FlowSchemaStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(
        &mut self,
        other: FlowSchemaStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
