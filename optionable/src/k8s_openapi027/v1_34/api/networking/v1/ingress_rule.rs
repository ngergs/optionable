#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressRuleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<
        <::k8s_openapi027::api::networking::v1::HTTPIngressRuleValue as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IngressRule {
    type Optioned = IngressRuleAc;
}
#[automatically_derived]
impl crate::Optionable for IngressRuleAc {
    type Optioned = IngressRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::IngressRule {
    fn into_optioned(self) -> IngressRuleAc {
        IngressRuleAc {
            host: self.host,
            http: crate::OptionableConvert::into_optioned(self.http),
        }
    }
    fn try_from_optioned(value: IngressRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            host: value.host,
            http: crate::OptionableConvert::try_from_optioned(value.http)?,
        })
    }
    fn merge(&mut self, other: IngressRuleAc) -> Result<(), crate::Error> {
        self.host = other.host;
        crate::OptionableConvert::merge(&mut self.http, other.http)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IngressRule>
for IngressRuleAc {
    fn from_optionable(value: k8s_openapi027::api::networking::v1::IngressRule) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::IngressRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
