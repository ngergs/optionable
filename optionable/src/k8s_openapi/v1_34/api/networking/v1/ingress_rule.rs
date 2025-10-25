pub struct IngressRuleOpt {
    pub host: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub http: <Option<
        ::k8s_openapi::api::networking::v1::HTTPIngressRuleValue,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::networking::v1::ingress_rule::IngressRule {
    type Optioned = IngressRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressRuleOpt {
    type Optioned = IngressRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::ingress_rule::IngressRule {
    fn into_optioned(self) -> IngressRuleOpt {
        IngressRuleOpt {
            host: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.host),
            http: <Option<
                ::k8s_openapi::api::networking::v1::HTTPIngressRuleValue,
            > as crate::OptionableConvert>::into_optioned(self.http),
        }
    }
    fn try_from_optioned(
        value: IngressRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            host: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.host)?,
            http: <Option<
                ::k8s_openapi::api::networking::v1::HTTPIngressRuleValue,
            > as crate::OptionableConvert>::try_from_optioned(value.http)?,
        })
    }
    fn merge(&mut self, other: IngressRuleOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.host, other.host)?;
        <Option<
            ::k8s_openapi::api::networking::v1::HTTPIngressRuleValue,
        > as crate::OptionableConvert>::merge(&mut self.http, other.http)?;
        Ok(())
    }
}
