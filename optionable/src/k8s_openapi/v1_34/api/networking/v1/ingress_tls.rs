pub struct IngressTLSOpt {
    pub hosts: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub secret_name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressTLS {
    type Optioned = IngressTLSOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressTLSOpt {
    type Optioned = IngressTLSOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressTLS {
    fn into_optioned(self) -> IngressTLSOpt {
        IngressTLSOpt {
            hosts: crate::OptionableConvert::into_optioned(self.hosts),
            secret_name: crate::OptionableConvert::into_optioned(self.secret_name),
        }
    }
    fn try_from_optioned(
        value: IngressTLSOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hosts: crate::OptionableConvert::try_from_optioned(value.hosts)?,
            secret_name: crate::OptionableConvert::try_from_optioned(value.secret_name)?,
        })
    }
    fn merge(&mut self, other: IngressTLSOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.hosts, other.hosts)?;
        crate::OptionableConvert::merge(&mut self.secret_name, other.secret_name)?;
        Ok(())
    }
}
