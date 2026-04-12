#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// LimitResponse defines how to handle requests that can not be executed right now.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LimitResponseAc {
    /// `queuing` holds the configuration parameters for queuing. This field may be non-empty only if `type` is `"Queue"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queuing: Option<
        <::k8s_openapi027::api::flowcontrol::v1::QueuingConfiguration as crate::Optionable>::Optioned,
    >,
    /// `type` is "Queue" or "Reject". "Queue" means that requests that can not be executed upon arrival are held in a queue until they can be executed or a queuing limit is reached. "Reject" means that requests that can not be executed upon arrival are rejected. Required.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::flowcontrol::v1::LimitResponse {
    type Optioned = LimitResponseAc;
}
#[automatically_derived]
impl crate::Optionable for LimitResponseAc {
    type Optioned = LimitResponseAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::flowcontrol::v1::LimitResponse {
    fn into_optioned(self) -> LimitResponseAc {
        LimitResponseAc {
            queuing: crate::OptionableConvert::into_optioned(self.queuing),
            type_: Some(self.type_),
        }
    }
    fn try_from_optioned(value: LimitResponseAc) -> Result<Self, crate::Error> {
        Ok(Self {
            queuing: crate::OptionableConvert::try_from_optioned(value.queuing)?,
            type_: value
                .type_
                .ok_or(crate::Error {
                    missing_field: "type_",
                })?,
        })
    }
    fn merge(&mut self, other: LimitResponseAc) -> Result<(), crate::Error> {
        if self.queuing.is_none() {
            self.queuing = crate::OptionableConvert::try_from_optioned(other.queuing)?;
        } else if let Some(self_value) = self.queuing.as_mut()
            && let Some(other_value) = other.queuing
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.type_ {
            self.type_ = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::flowcontrol::v1::LimitResponse>
for LimitResponseAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::LimitResponse,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::flowcontrol::v1::LimitResponse, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::LimitResponse,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
