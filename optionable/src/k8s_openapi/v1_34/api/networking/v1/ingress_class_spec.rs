#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressClassSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: <Option<
        ::k8s_openapi::api::networking::v1::IngressClassParametersReference,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressClassSpec {
    type Optioned = IngressClassSpecAc;
}
#[automatically_derived]
impl crate::Optionable for IngressClassSpecAc {
    type Optioned = IngressClassSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressClassSpec {
    fn into_optioned(self) -> IngressClassSpecAc {
        IngressClassSpecAc {
            controller: crate::OptionableConvert::into_optioned(self.controller),
            parameters: crate::OptionableConvert::into_optioned(self.parameters),
        }
    }
    fn try_from_optioned(
        value: IngressClassSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            controller: crate::OptionableConvert::try_from_optioned(value.controller)?,
            parameters: crate::OptionableConvert::try_from_optioned(value.parameters)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressClassSpecAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.controller, other.controller)?;
        crate::OptionableConvert::merge(&mut self.parameters, other.parameters)?;
        Ok(())
    }
}
