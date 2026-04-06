#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParamRefAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_not_found_action: Option<std::string::String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1alpha1::ParamRef {
    type Optioned = ParamRefAc;
}
#[automatically_derived]
impl crate::Optionable for ParamRefAc {
    type Optioned = ParamRefAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1alpha1::ParamRef {
    fn into_optioned(self) -> ParamRefAc {
        ParamRefAc {
            name: self.name,
            namespace: self.namespace,
            parameter_not_found_action: self.parameter_not_found_action,
            selector: crate::OptionableConvert::into_optioned(self.selector),
        }
    }
    fn try_from_optioned(value: ParamRefAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: value.name,
            namespace: value.namespace,
            parameter_not_found_action: value.parameter_not_found_action,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
        })
    }
    fn merge(&mut self, other: ParamRefAc) -> Result<(), crate::Error> {
        self.name = other.name;
        self.namespace = other.namespace;
        self.parameter_not_found_action = other.parameter_not_found_action;
        crate::OptionableConvert::merge(&mut self.selector, other.selector)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1alpha1::ParamRef,
> for ParamRefAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1alpha1::ParamRef,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1alpha1::ParamRef,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1alpha1::ParamRef,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
