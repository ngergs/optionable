pub struct JSONPatchOpt {
    pub expression: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1alpha1::JSONPatch {
    type Optioned = JSONPatchOpt;
}
#[automatically_derived]
impl crate::Optionable for JSONPatchOpt {
    type Optioned = JSONPatchOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1alpha1::JSONPatch {
    fn into_optioned(self) -> JSONPatchOpt {
        JSONPatchOpt {
            expression: crate::OptionableConvert::into_optioned(self.expression),
        }
    }
    fn try_from_optioned(value: JSONPatchOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression: crate::OptionableConvert::try_from_optioned(value.expression)?,
        })
    }
    fn merge(&mut self, other: JSONPatchOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.expression, other.expression)?;
        Ok(())
    }
}
