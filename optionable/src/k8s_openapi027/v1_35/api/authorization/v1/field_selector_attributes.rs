#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// FieldSelectorAttributes indicates a field limited access. Webhook authors are encouraged to * ensure rawSelector and requirements are not both set * consider the requirements field if set * not try to parse or consider the rawSelector field if set. This is to avoid another CVE-2022-2880 (i.e. getting different systems to agree on how exactly to parse a query is not something we want), see https://www.oxeye.io/resources/golang-parameter-smuggling-attack for more details. For the *SubjectAccessReview endpoints of the kube-apiserver: * If rawSelector is empty and requirements are empty, the request is not limited. * If rawSelector is present and requirements are empty, the rawSelector will be parsed and limited if the parsing succeeds. * If rawSelector is empty and requirements are present, the requirements should be honored * If rawSelector is present and requirements are present, the request is invalid.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldSelectorAttributesAc {
    /// rawSelector is the serialization of a field selector that would be included in a query parameter. Webhook implementations are encouraged to ignore rawSelector. The kube-apiserver's *SubjectAccessReview will parse the rawSelector as long as the requirements are not present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_selector: Option<std::string::String>,
    /// requirements is the parsed interpretation of a field selector. All requirements must be met for a resource instance to match the selector. Webhook implementations should handle requirements, but how to handle them is up to the webhook. Since requirements can only limit the request, it is safe to authorize as unlimited request if the requirements are not understood.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<
        std::vec::Vec<
            <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::FieldSelectorRequirement as crate::Optionable>::Optioned,
        >,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::authorization::v1::FieldSelectorAttributes {
    type Optioned = FieldSelectorAttributesAc;
}
#[automatically_derived]
impl crate::Optionable for FieldSelectorAttributesAc {
    type Optioned = FieldSelectorAttributesAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::authorization::v1::FieldSelectorAttributes {
    fn into_optioned(self) -> FieldSelectorAttributesAc {
        FieldSelectorAttributesAc {
            raw_selector: self.raw_selector,
            requirements: crate::OptionableConvert::into_optioned(self.requirements),
        }
    }
    fn try_from_optioned(
        value: FieldSelectorAttributesAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            raw_selector: value.raw_selector,
            requirements: crate::OptionableConvert::try_from_optioned(
                value.requirements,
            )?,
        })
    }
    fn merge(&mut self, other: FieldSelectorAttributesAc) -> Result<(), crate::Error> {
        if self.raw_selector.is_none() {
            self.raw_selector = crate::OptionableConvert::try_from_optioned(
                other.raw_selector,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.raw_selector, other.raw_selector)?;
        }
        if self.requirements.is_none() {
            self.requirements = crate::OptionableConvert::try_from_optioned(
                other.requirements,
            )?;
        } else {
            self.requirements = crate::OptionableConvert::try_from_optioned(
                other.requirements,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::authorization::v1::FieldSelectorAttributes,
> for FieldSelectorAttributesAc {
    fn from_optionable(
        value: k8s_openapi027::api::authorization::v1::FieldSelectorAttributes,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::authorization::v1::FieldSelectorAttributes,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::authorization::v1::FieldSelectorAttributes,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
