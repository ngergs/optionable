pub struct ParamRefOpt {
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub parameter_not_found_action: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub selector: <Option<
        ::k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::ParamRef {
    type Optioned = ParamRefOpt;
}
#[automatically_derived]
impl crate::Optionable for ParamRefOpt {
    type Optioned = ParamRefOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::ParamRef {
    fn into_optioned(self) -> ParamRefOpt {
        ParamRefOpt {
            name: crate::OptionableConvert::into_optioned(self.name),
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
            parameter_not_found_action: crate::OptionableConvert::into_optioned(
                self.parameter_not_found_action,
            ),
            selector: crate::OptionableConvert::into_optioned(self.selector),
        }
    }
    fn try_from_optioned(value: ParamRefOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
            parameter_not_found_action: crate::OptionableConvert::try_from_optioned(
                value.parameter_not_found_action,
            )?,
            selector: crate::OptionableConvert::try_from_optioned(value.selector)?,
        })
    }
    fn merge(&mut self, other: ParamRefOpt) -> Result<(), crate::optionable::Error> {
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
