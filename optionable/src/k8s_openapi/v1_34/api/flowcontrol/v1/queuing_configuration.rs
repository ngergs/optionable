pub struct QueuingConfigurationOpt {
    pub hand_size: <Option<i32> as crate::Optionable>::Optioned,
    pub queue_length_limit: <Option<i32> as crate::Optionable>::Optioned,
    pub queues: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::queuing_configuration::QueuingConfiguration {
    type Optioned = QueuingConfigurationOpt;
}
#[automatically_derived]
impl crate::Optionable for QueuingConfigurationOpt {
    type Optioned = QueuingConfigurationOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::queuing_configuration::QueuingConfiguration {
    fn into_optioned(self) -> QueuingConfigurationOpt {
        QueuingConfigurationOpt {
            hand_size: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.hand_size),
            queue_length_limit: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.queue_length_limit),
            queues: <Option<i32> as crate::OptionableConvert>::into_optioned(self.queues),
        }
    }
    fn try_from_optioned(
        value: QueuingConfigurationOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hand_size: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.hand_size)?,
            queue_length_limit: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.queue_length_limit)?,
            queues: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.queues)?,
        })
    }
    fn merge(
        &mut self,
        other: QueuingConfigurationOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.hand_size, other.hand_size)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.queue_length_limit,
            other.queue_length_limit,
        )?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.queues, other.queues)?;
        Ok(())
    }
}
