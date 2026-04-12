#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// NonResourceAttributes includes the authorization attributes available for non-resource requests to the Authorizer interface
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NonResourceAttributesAc {
    /// Path is the URL path of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// Verb is the standard HTTP verb
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::authorization::v1::NonResourceAttributes {
    type Optioned = NonResourceAttributesAc;
}
#[automatically_derived]
impl crate::Optionable for NonResourceAttributesAc {
    type Optioned = NonResourceAttributesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authorization::v1::NonResourceAttributes {
    fn into_optioned(self) -> NonResourceAttributesAc {
        NonResourceAttributesAc {
            path: self.path,
            verb: self.verb,
        }
    }
    fn try_from_optioned(value: NonResourceAttributesAc) -> Result<Self, crate::Error> {
        Ok(Self {
            path: value.path,
            verb: value.verb,
        })
    }
    fn merge(&mut self, other: NonResourceAttributesAc) -> Result<(), crate::Error> {
        if self.path.is_none() {
            self.path = other.path;
        }
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        if self.verb.is_none() {
            self.verb = other.verb;
        }
        if let Some(other_value) = other.verb {
            crate::OptionableConvert::merge(&mut self.verb, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::authorization::v1::NonResourceAttributes,
> for NonResourceAttributesAc {
    fn from_optionable(
        value: k8s_openapi027::api::authorization::v1::NonResourceAttributes,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authorization::v1::NonResourceAttributes,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authorization::v1::NonResourceAttributes,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
