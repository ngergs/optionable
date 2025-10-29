pub struct IngressRuleOpt {
    pub host: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub http: <Option<
        ::k8s_openapi::api::networking::v1::HTTPIngressRuleValue,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressRule {
    type Optioned = IngressRuleOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressRuleOpt {
    type Optioned = IngressRuleOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressRule {
    fn into_optioned(self) -> IngressRuleOpt {
        IngressRuleOpt {
            host: crate::OptionableConvert::into_optioned(self.host),
            http: crate::OptionableConvert::into_optioned(self.http),
        }
    }
    fn try_from_optioned(
        value: IngressRuleOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            host: crate::OptionableConvert::try_from_optioned(value.host)?,
            http: crate::OptionableConvert::try_from_optioned(value.http)?,
        })
    }
    fn merge(&mut self, other: IngressRuleOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.host, other.host)?;
        crate::OptionableConvert::merge(&mut self.http, other.http)?;
        Ok(())
    }
}
