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
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_not_found_action: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: <Option<
        ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::admissionregistration::v1beta1::ParamRef {
    type Optioned = ParamRefAc;
}
#[automatically_derived]
impl crate::Optionable for ParamRefAc {
    type Optioned = ParamRefAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::admissionregistration::v1beta1::ParamRef {
    fn into_optioned(self) -> ParamRefAc {
        ParamRefAc {
            name: crate::OptionableConvert::into_optioned(self.name),
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
            parameter_not_found_action: crate::OptionableConvert::into_optioned(
                self.parameter_not_found_action,
            ),
            selector: crate::OptionableConvert::into_optioned(self.selector),
        }
    }
    fn try_from_optioned(value: ParamRefAc) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
            parameter_not_found_action: crate::OptionableConvert::try_from_optioned(
                value.parameter_not_found_action,
            )?,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
        })
    }
    fn merge(&mut self, other: ParamRefAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        crate::OptionableConvert::merge(
            &mut self.parameter_not_found_action,
            other.parameter_not_found_action,
        )?;
        crate::OptionableConvert::merge(&mut self.selector, other.selector)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::admissionregistration::v1beta1::ParamRef,
> for ParamRefAc {
    fn from_optionable(
        value: k8s_openapi027::api::admissionregistration::v1beta1::ParamRef,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::admissionregistration::v1beta1::ParamRef,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::admissionregistration::v1beta1::ParamRef,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
