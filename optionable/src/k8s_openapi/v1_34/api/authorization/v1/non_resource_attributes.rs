pub struct NonResourceAttributesAc {
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub verb: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authorization::v1::NonResourceAttributes {
    type Optioned = NonResourceAttributesAc;
}
#[automatically_derived]
impl crate::Optionable for NonResourceAttributesAc {
    type Optioned = NonResourceAttributesAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::NonResourceAttributes {
    fn into_optioned(self) -> NonResourceAttributesAc {
        NonResourceAttributesAc {
            path: crate::OptionableConvert::into_optioned(self.path),
            verb: crate::OptionableConvert::into_optioned(self.verb),
        }
    }
    fn try_from_optioned(
        value: NonResourceAttributesAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            path: crate::OptionableConvert::try_from_optioned(value.path)?,
            verb: crate::OptionableConvert::try_from_optioned(value.verb)?,
        })
    }
    fn merge(
        &mut self,
        other: NonResourceAttributesAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.path, other.path)?;
        crate::OptionableConvert::merge(&mut self.verb, other.verb)?;
        Ok(())
    }
}
