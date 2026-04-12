#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ClientIPConfig represents the configurations of Client IP based session affinity.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClientIPConfigAc {
    /// timeoutSeconds specifies the seconds of ClientIP type session sticky time. The value must be \>0 && \<=86400(for 1 day) if ServiceAffinity == "ClientIP". Default value is 10800(for 3 hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ClientIPConfig {
    type Optioned = ClientIPConfigAc;
}
#[automatically_derived]
impl crate::Optionable for ClientIPConfigAc {
    type Optioned = ClientIPConfigAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ClientIPConfig {
    fn into_optioned(self) -> ClientIPConfigAc {
        ClientIPConfigAc {
            timeout_seconds: self.timeout_seconds,
        }
    }
    fn try_from_optioned(value: ClientIPConfigAc) -> Result<Self, crate::Error> {
        Ok(Self {
            timeout_seconds: value.timeout_seconds,
        })
    }
    fn merge(&mut self, other: ClientIPConfigAc) -> Result<(), crate::Error> {
        if other.timeout_seconds.is_some() {
            self.timeout_seconds = other.timeout_seconds;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ClientIPConfig>
for ClientIPConfigAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ClientIPConfig) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ClientIPConfig, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ClientIPConfig,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
