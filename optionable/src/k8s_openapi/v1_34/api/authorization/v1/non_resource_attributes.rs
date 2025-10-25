pub struct NonResourceAttributesOpt {
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub verb: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authorization::v1::NonResourceAttributes {
    type Optioned = NonResourceAttributesOpt;
}
#[automatically_derived]
impl crate::Optionable for NonResourceAttributesOpt {
    type Optioned = NonResourceAttributesOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::NonResourceAttributes {
    fn into_optioned(self) -> NonResourceAttributesOpt {
        NonResourceAttributesOpt {
            path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.path),
            verb: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.verb),
        }
    }
    fn try_from_optioned(
        value: NonResourceAttributesOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.path)?,
            verb: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.verb)?,
        })
    }
    fn merge(
        &mut self,
        other: NonResourceAttributesOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.path, other.path)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.verb, other.verb)?;
        Ok(())
    }
}
