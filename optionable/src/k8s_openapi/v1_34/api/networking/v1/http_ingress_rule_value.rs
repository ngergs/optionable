pub struct HTTPIngressRuleValueAc {
    pub paths: Option<
        <std::vec::Vec<
            ::k8s_openapi::api::networking::v1::HTTPIngressPath,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::HTTPIngressRuleValue {
    type Optioned = HTTPIngressRuleValueAc;
}
#[automatically_derived]
impl crate::Optionable for HTTPIngressRuleValueAc {
    type Optioned = HTTPIngressRuleValueAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::HTTPIngressRuleValue {
    fn into_optioned(self) -> HTTPIngressRuleValueAc {
        HTTPIngressRuleValueAc {
            paths: Some(crate::OptionableConvert::into_optioned(self.paths)),
        }
    }
    fn try_from_optioned(
        value: HTTPIngressRuleValueAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            paths: crate::OptionableConvert::try_from_optioned(
                value
                    .paths
                    .ok_or(crate::optionable::Error {
                        missing_field: "paths",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: HTTPIngressRuleValueAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.paths {
            crate::OptionableConvert::merge(&mut self.paths, other_value)?;
        }
        Ok(())
    }
}
