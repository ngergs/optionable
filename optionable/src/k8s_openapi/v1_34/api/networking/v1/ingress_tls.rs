#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressTLSAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressTLS {
    type Optioned = IngressTLSAc;
}
#[automatically_derived]
impl crate::Optionable for IngressTLSAc {
    type Optioned = IngressTLSAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressTLS {
    fn into_optioned(self) -> IngressTLSAc {
        IngressTLSAc {
            hosts: crate::OptionableConvert::into_optioned(self.hosts),
            secret_name: crate::OptionableConvert::into_optioned(self.secret_name),
        }
    }
    fn try_from_optioned(value: IngressTLSAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            hosts: crate::OptionableConvert::try_from_optioned(value.hosts)?,
            secret_name: crate::OptionableConvert::try_from_optioned(value.secret_name)?,
        })
    }
    fn merge(&mut self, other: IngressTLSAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.hosts, other.hosts)?;
        crate::OptionableConvert::merge(&mut self.secret_name, other.secret_name)?;
        Ok(())
    }
}
