#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// FlowDistinguisherMethod specifies the method of a flow distinguisher.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FlowDistinguisherMethodAc {
    /// `type` is the type of flow distinguisher method The supported types are "ByUser" and "ByNamespace". Required.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1beta3::FlowDistinguisherMethod {
    type Optioned = FlowDistinguisherMethodAc;
}
#[automatically_derived]
impl crate::Optionable for FlowDistinguisherMethodAc {
    type Optioned = FlowDistinguisherMethodAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1beta3::FlowDistinguisherMethod {
    fn into_optioned(self) -> FlowDistinguisherMethodAc {
        FlowDistinguisherMethodAc {
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(
        value: FlowDistinguisherMethodAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: FlowDistinguisherMethodAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.type_ {
            self.type_ = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::flowcontrol::v1beta3::FlowDistinguisherMethod,
> for FlowDistinguisherMethodAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1beta3::FlowDistinguisherMethod,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1beta3::FlowDistinguisherMethod,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1beta3::FlowDistinguisherMethod,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
