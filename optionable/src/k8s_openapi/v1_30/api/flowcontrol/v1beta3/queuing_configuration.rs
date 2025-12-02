#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct QueuingConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_size: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_length_limit: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1beta3::QueuingConfiguration {
    type Optioned = QueuingConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for QueuingConfigurationAc {
    type Optioned = QueuingConfigurationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1beta3::QueuingConfiguration {
    fn into_optioned(self) -> QueuingConfigurationAc {
        QueuingConfigurationAc {
            hand_size: crate::OptionableConvert::into_optioned(self.hand_size),
            queue_length_limit: crate::OptionableConvert::into_optioned(
                self.queue_length_limit,
            ),
            queues: crate::OptionableConvert::into_optioned(self.queues),
        }
    }
    fn try_from_optioned(value: QueuingConfigurationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hand_size: crate::OptionableConvert::try_from_optioned(value.hand_size)?,
            queue_length_limit: crate::OptionableConvert::try_from_optioned(
                value.queue_length_limit,
            )?,
            queues: crate::OptionableConvert::try_from_optioned(value.queues)?,
        })
    }
    fn merge(&mut self, other: QueuingConfigurationAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.hand_size, other.hand_size)?;
        crate::OptionableConvert::merge(
            &mut self.queue_length_limit,
            other.queue_length_limit,
        )?;
        crate::OptionableConvert::merge(&mut self.queues, other.queues)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    ::k8s_openapi::api::flowcontrol::v1beta3::QueuingConfiguration,
> for QueuingConfigurationAc {
    fn from_optionable(
        value: ::k8s_openapi::api::flowcontrol::v1beta3::QueuingConfiguration,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        ::k8s_openapi::api::flowcontrol::v1beta3::QueuingConfiguration,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::flowcontrol::v1beta3::QueuingConfiguration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
