#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NonResourceAttributesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi026::api::authorization::v1::NonResourceAttributes {
    type Optioned = NonResourceAttributesAc;
}
#[automatically_derived]
impl crate::Optionable for NonResourceAttributesAc {
    type Optioned = NonResourceAttributesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::authorization::v1::NonResourceAttributes {
    fn into_optioned(self) -> NonResourceAttributesAc {
        NonResourceAttributesAc {
            path: crate::OptionableConvert::into_optioned(self.path),
            verb: crate::OptionableConvert::into_optioned(self.verb),
        }
    }
    fn try_from_optioned(value: NonResourceAttributesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            path: crate::OptionableConvert::try_from_optioned(value.path)?,
            verb: crate::OptionableConvert::try_from_optioned(value.verb)?,
        })
    }
    fn merge(&mut self, other: NonResourceAttributesAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.path, other.path)?;
        crate::OptionableConvert::merge(&mut self.verb, other.verb)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi026::api::authorization::v1::NonResourceAttributes,
> for NonResourceAttributesAc {
    fn from_optionable(
        value: k8s_openapi026::api::authorization::v1::NonResourceAttributes,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::authorization::v1::NonResourceAttributes,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::authorization::v1::NonResourceAttributes,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
