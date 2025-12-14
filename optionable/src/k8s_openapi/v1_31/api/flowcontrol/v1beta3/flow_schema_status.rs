#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowSchemaStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: <Option<
        std::vec::Vec<::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaCondition>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaStatus {
    type Optioned = FlowSchemaStatusAc;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaStatusAc {
    type Optioned = FlowSchemaStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaStatus {
    fn into_optioned(self) -> FlowSchemaStatusAc {
        FlowSchemaStatusAc {
            conditions: crate::OptionableConvert::into_optioned(self.conditions),
        }
    }
    fn try_from_optioned(value: FlowSchemaStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: crate::OptionableConvert::try_from_optioned(value.conditions)?,
        })
    }
    fn merge(&mut self, other: FlowSchemaStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.conditions, other.conditions)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaStatus>
for FlowSchemaStatusAc {
    fn from_optionable(
        value: ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaStatus,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::flowcontrol::v1beta3::FlowSchemaStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
