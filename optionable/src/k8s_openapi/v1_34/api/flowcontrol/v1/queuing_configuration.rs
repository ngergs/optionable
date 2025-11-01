pub struct QueuingConfigurationAc {
    pub hand_size: <Option<i32> as crate::Optionable>::Optioned,
    pub queue_length_limit: <Option<i32> as crate::Optionable>::Optioned,
    pub queues: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::QueuingConfiguration {
    type Optioned = QueuingConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for QueuingConfigurationAc {
    type Optioned = QueuingConfigurationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::QueuingConfiguration {
    fn into_optioned(self) -> QueuingConfigurationAc {
        QueuingConfigurationAc {
            hand_size: crate::OptionableConvert::into_optioned(self.hand_size),
            queue_length_limit: crate::OptionableConvert::into_optioned(
                self.queue_length_limit,
            ),
            queues: crate::OptionableConvert::into_optioned(self.queues),
        }
    }
    fn try_from_optioned(
        value: QueuingConfigurationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hand_size: crate::OptionableConvert::try_from_optioned(value.hand_size)?,
            queue_length_limit: crate::OptionableConvert::try_from_optioned(
                value.queue_length_limit,
            )?,
            queues: crate::OptionableConvert::try_from_optioned(value.queues)?,
        })
    }
    fn merge(
        &mut self,
        other: QueuingConfigurationAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.hand_size, other.hand_size)?;
        crate::OptionableConvert::merge(
            &mut self.queue_length_limit,
            other.queue_length_limit,
        )?;
        crate::OptionableConvert::merge(&mut self.queues, other.queues)?;
        Ok(())
    }
}
