#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// IngressClassSpec provides information about the class of an Ingress.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IngressClassSpecAc {
    /// controller refers to the name of the controller that should handle this class. This allows for different "flavors" that are controlled by the same controller. For example, you may have different parameters for the same implementing controller. This should be specified as a domain-prefixed path no more than 250 characters in length, e.g. "acme.io/ingress-controller". This field is immutable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<std::string::String>,
    /// parameters is a link to a custom resource containing additional configuration for the controller. This is optional if the controller does not require extra parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<
        <::k8s_openapi027::api::networking::v1::IngressClassParametersReference as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::IngressClassSpec {
    type Optioned = IngressClassSpecAc;
}
#[automatically_derived]
impl crate::Optionable for IngressClassSpecAc {
    type Optioned = IngressClassSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::IngressClassSpec {
    fn into_optioned(self) -> IngressClassSpecAc {
        IngressClassSpecAc {
            controller: self.controller,
            parameters: crate::OptionableConvert::into_optioned(self.parameters),
        }
    }
    fn try_from_optioned(value: IngressClassSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            controller: value.controller,
            parameters: crate::OptionableConvert::try_from_optioned(value.parameters)?,
        })
    }
    fn merge(&mut self, other: IngressClassSpecAc) -> Result<(), crate::Error> {
        self.controller = other.controller;
        crate::OptionableConvert::merge(&mut self.parameters, other.parameters)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::IngressClassSpec>
for IngressClassSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::IngressClassSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::IngressClassSpec, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::IngressClassSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
