pub struct JSONPatchAc {
    pub expression: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1beta1::JSONPatch {
    type Optioned = JSONPatchAc;
}
#[automatically_derived]
impl crate::Optionable for JSONPatchAc {
    type Optioned = JSONPatchAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1beta1::JSONPatch {
    fn into_optioned(self) -> JSONPatchAc {
        JSONPatchAc {
            expression: crate::OptionableConvert::into_optioned(self.expression),
        }
    }
    fn try_from_optioned(value: JSONPatchAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression: crate::OptionableConvert::try_from_optioned(value.expression)?,
        })
    }
    fn merge(&mut self, other: JSONPatchAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.expression, other.expression)?;
        Ok(())
    }
}
