#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// FlowSchemaStatus represents the current state of a FlowSchema.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowSchemaStatusAc {
    /// `conditions` is a list of the current states of FlowSchema.
    pub conditions: Option<
        std::vec::Vec<::k8s_openapi027::api::flowcontrol::v1::FlowSchemaCondition>,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::flowcontrol::v1::FlowSchemaStatus {
    type Optioned = FlowSchemaStatusAc;
}
#[automatically_derived]
impl crate::Optionable for FlowSchemaStatusAc {
    type Optioned = FlowSchemaStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1::FlowSchemaStatus {
    fn into_optioned(self) -> FlowSchemaStatusAc {
        FlowSchemaStatusAc {
            conditions: self.conditions,
        }
    }
    fn try_from_optioned(value: FlowSchemaStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            conditions: value.conditions,
        })
    }
    fn merge(&mut self, other: FlowSchemaStatusAc) -> Result<(), crate::Error> {
        self.conditions = other.conditions;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::flowcontrol::v1::FlowSchemaStatus>
for FlowSchemaStatusAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::FlowSchemaStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::flowcontrol::v1::FlowSchemaStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::FlowSchemaStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
