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
    pub effect: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toleration_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::Toleration {
    type Optioned = TolerationAc;
}
#[automatically_derived]
impl crate::Optionable for TolerationAc {
    type Optioned = TolerationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::Toleration {
    fn into_optioned(self) -> TolerationAc {
        TolerationAc {
            effect: self.effect,
            key: self.key,
            operator: self.operator,
            toleration_seconds: self.toleration_seconds,
            value: self.value,
        }
    }
    fn try_from_optioned(value: TolerationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            effect: value.effect,
            key: value.key,
            operator: value.operator,
            toleration_seconds: value.toleration_seconds,
            value: value.value,
        })
    }
    fn merge(&mut self, other: TolerationAc) -> Result<(), crate::Error> {
        self.effect = other.effect;
        self.key = other.key;
        self.operator = other.operator;
        self.toleration_seconds = other.toleration_seconds;
        self.value = other.value;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::Toleration> for TolerationAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::Toleration) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::Toleration, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::Toleration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
