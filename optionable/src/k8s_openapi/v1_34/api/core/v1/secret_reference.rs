pub struct SecretReferenceAc {
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::SecretReference {
    type Optioned = SecretReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for SecretReferenceAc {
    type Optioned = SecretReferenceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::SecretReference {
    fn into_optioned(self) -> SecretReferenceAc {
        SecretReferenceAc {
            name: crate::OptionableConvert::into_optioned(self.name),
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
        }
    }
    fn try_from_optioned(
        value: SecretReferenceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
        })
    }
    fn merge(
        &mut self,
        other: SecretReferenceAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        Ok(())
    }
}
