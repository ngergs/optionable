#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceTolerationAc {
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
impl crate::Optionable for k8s_openapi027::api::resource::v1::DeviceToleration {
    type Optioned = DeviceTolerationAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceTolerationAc {
    type Optioned = DeviceTolerationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1::DeviceToleration {
    fn into_optioned(self) -> DeviceTolerationAc {
        DeviceTolerationAc {
            effect: self.effect,
            key: self.key,
            operator: self.operator,
            toleration_seconds: self.toleration_seconds,
            value: self.value,
        }
    }
    fn try_from_optioned(value: DeviceTolerationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            effect: value.effect,
            key: value.key,
            operator: value.operator,
            toleration_seconds: value.toleration_seconds,
            value: value.value,
        })
    }
    fn merge(&mut self, other: DeviceTolerationAc) -> Result<(), crate::Error> {
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
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1::DeviceToleration>
for DeviceTolerationAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1::DeviceToleration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1::DeviceToleration, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1::DeviceToleration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
