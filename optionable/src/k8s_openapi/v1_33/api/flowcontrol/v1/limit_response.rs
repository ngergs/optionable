#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LimitResponseAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queuing: <Option<
        ::k8s_openapi::api::flowcontrol::v1::QueuingConfiguration,
    > as crate::Optionable>::Optioned,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::LimitResponse {
    type Optioned = LimitResponseAc;
}
#[automatically_derived]
impl crate::Optionable for LimitResponseAc {
    type Optioned = LimitResponseAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::flowcontrol::v1::LimitResponse {
    fn into_optioned(self) -> LimitResponseAc {
        LimitResponseAc {
            queuing: crate::OptionableConvert::into_optioned(self.queuing),
            type_: Some(crate::OptionableConvert::into_optioned(self.type_)),
        }
    }
    fn try_from_optioned(value: LimitResponseAc) -> Result<Self, crate::Error> {
        Ok(Self {
            queuing: crate::OptionableConvert::try_from_optioned(value.queuing)?,
            type_: crate::OptionableConvert::try_from_optioned(
                value
                    .type_
                    .ok_or(crate::Error {
                        missing_field: "type_",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: LimitResponseAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.queuing, other.queuing)?;
        if let Some(other_value) = other.type_ {
            crate::OptionableConvert::merge(&mut self.type_, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::flowcontrol::v1::LimitResponse>
for LimitResponseAc {
    fn from_optionable(
        value: ::k8s_openapi::api::flowcontrol::v1::LimitResponse,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::flowcontrol::v1::LimitResponse, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::flowcontrol::v1::LimitResponse,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
