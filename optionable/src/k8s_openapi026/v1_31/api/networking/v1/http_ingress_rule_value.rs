#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HTTPIngressRuleValueAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<
        <std::vec::Vec<
            ::k8s_openapi026::api::networking::v1::HTTPIngressPath,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::networking::v1::HTTPIngressRuleValue {
    type Optioned = HTTPIngressRuleValueAc;
}
#[automatically_derived]
impl crate::Optionable for HTTPIngressRuleValueAc {
    type Optioned = HTTPIngressRuleValueAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::networking::v1::HTTPIngressRuleValue {
    fn into_optioned(self) -> HTTPIngressRuleValueAc {
        HTTPIngressRuleValueAc {
            paths: Some(crate::OptionableConvert::into_optioned(self.paths)),
        }
    }
    fn try_from_optioned(value: HTTPIngressRuleValueAc) -> Result<Self, crate::Error> {
        Ok(Self {
            paths: crate::OptionableConvert::try_from_optioned(
                value
                    .paths
                    .ok_or(crate::Error {
                        missing_field: "paths",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: HTTPIngressRuleValueAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.paths {
            crate::OptionableConvert::merge(&mut self.paths, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::networking::v1::HTTPIngressRuleValue>
for HTTPIngressRuleValueAc {
    fn from_optionable(
        value: k8s_openapi026::api::networking::v1::HTTPIngressRuleValue,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi026::api::networking::v1::HTTPIngressRuleValue,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::networking::v1::HTTPIngressRuleValue,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
