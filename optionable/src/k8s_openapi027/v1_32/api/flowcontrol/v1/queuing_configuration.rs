#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QueuingConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_length_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::flowcontrol::v1::QueuingConfiguration {
    type Optioned = QueuingConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for QueuingConfigurationAc {
    type Optioned = QueuingConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1::QueuingConfiguration {
    fn into_optioned(self) -> QueuingConfigurationAc {
        QueuingConfigurationAc {
            hand_size: self.hand_size,
            queue_length_limit: self.queue_length_limit,
            queues: self.queues,
        }
    }
    fn try_from_optioned(value: QueuingConfigurationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hand_size: value.hand_size,
            queue_length_limit: value.queue_length_limit,
            queues: value.queues,
        })
    }
    fn merge(&mut self, other: QueuingConfigurationAc) -> Result<(), crate::Error> {
        self.hand_size = other.hand_size;
        self.queue_length_limit = other.queue_length_limit;
        self.queues = other.queues;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::flowcontrol::v1::QueuingConfiguration>
for QueuingConfigurationAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1::QueuingConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1::QueuingConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1::QueuingConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
