#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TolerationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toleration_seconds: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::Toleration {
    type Optioned = TolerationAc;
}
#[automatically_derived]
impl crate::Optionable for TolerationAc {
    type Optioned = TolerationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::Toleration {
    fn into_optioned(self) -> TolerationAc {
        TolerationAc {
            effect: crate::OptionableConvert::into_optioned(self.effect),
            key: crate::OptionableConvert::into_optioned(self.key),
            operator: crate::OptionableConvert::into_optioned(self.operator),
            toleration_seconds: crate::OptionableConvert::into_optioned(
                self.toleration_seconds,
            ),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(value: TolerationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            effect: crate::OptionableConvert::try_from_optioned(value.effect)?,
            key: crate::OptionableConvert::try_from_optioned(value.key)?,
            operator: crate::OptionableConvert::try_from_optioned(value.operator)?,
            toleration_seconds: crate::OptionableConvert::try_from_optioned(
                value.toleration_seconds,
            )?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(&mut self, other: TolerationAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.effect, other.effect)?;
        crate::OptionableConvert::merge(&mut self.key, other.key)?;
        crate::OptionableConvert::merge(&mut self.operator, other.operator)?;
        crate::OptionableConvert::merge(
            &mut self.toleration_seconds,
            other.toleration_seconds,
        )?;
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::Toleration> for TolerationAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::Toleration) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::Toleration, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::Toleration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
