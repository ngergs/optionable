#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// QueuingConfiguration holds the configuration parameters for queuing
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct QueuingConfigurationAc {
    /// `handSize` is a small positive number that configures the shuffle sharding of requests into queues.  When enqueuing a request at this priority level the request's flow identifier (a string pair) is hashed and the hash value is used to shuffle the list of queues and deal a hand of the size specified here.  The request is put into one of the shortest queues in that hand. `handSize` must be no larger than `queues`, and should be significantly smaller (so that a few heavy flows do not saturate most of the queues).  See the user-facing documentation for more extensive guidance on setting this field.  This field has a default value of 8.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hand_size: Option<i32>,
    /// `queueLengthLimit` is the maximum number of requests allowed to be waiting in a given queue of this priority level at a time; excess requests are rejected.  This value must be positive.  If not specified, it will be defaulted to 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_length_limit: Option<i32>,
    /// `queues` is the number of queues for this priority level. The queues exist independently at each apiserver. The value must be positive.  Setting it to 1 effectively precludes shufflesharding and thus makes the distinguisher method of associated flow schemas irrelevant.  This field has a default value of 64.
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
