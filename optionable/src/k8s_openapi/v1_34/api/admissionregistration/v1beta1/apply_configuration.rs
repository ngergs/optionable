#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyConfigurationAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::admissionregistration::v1beta1::ApplyConfiguration {
    type Optioned = ApplyConfigurationAc;
}
#[automatically_derived]
impl crate::Optionable for ApplyConfigurationAc {
    type Optioned = ApplyConfigurationAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::admissionregistration::v1beta1::ApplyConfiguration {
    fn into_optioned(self) -> ApplyConfigurationAc {
        ApplyConfigurationAc {
            expression: crate::OptionableConvert::into_optioned(self.expression),
        }
    }
    fn try_from_optioned(
        value: ApplyConfigurationAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression: crate::OptionableConvert::try_from_optioned(value.expression)?,
        })
    }
    fn merge(
        &mut self,
        other: ApplyConfigurationAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.expression, other.expression)?;
        Ok(())
    }
}
