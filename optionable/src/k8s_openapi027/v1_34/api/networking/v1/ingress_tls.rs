#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressTLSAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IngressTLS {
    type Optioned = IngressTLSAc;
}
#[automatically_derived]
impl crate::Optionable for IngressTLSAc {
    type Optioned = IngressTLSAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::IngressTLS {
    fn into_optioned(self) -> IngressTLSAc {
        IngressTLSAc {
            hosts: crate::OptionableConvert::into_optioned(self.hosts),
            secret_name: crate::OptionableConvert::into_optioned(self.secret_name),
        }
    }
    fn try_from_optioned(value: IngressTLSAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hosts: crate::OptionableConvert::try_from_optioned(value.hosts)?,
            secret_name: crate::OptionableConvert::try_from_optioned(value.secret_name)?,
        })
    }
    fn merge(&mut self, other: IngressTLSAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.hosts, other.hosts)?;
        crate::OptionableConvert::merge(&mut self.secret_name, other.secret_name)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IngressTLS>
for IngressTLSAc {
    fn from_optionable(value: k8s_openapi027::api::networking::v1::IngressTLS) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::IngressTLS, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressTLS,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
