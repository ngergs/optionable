#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriorityLevelConfigurationReferenceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationReference {
    type Optioned = PriorityLevelConfigurationReferenceAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityLevelConfigurationReferenceAc {
    type Optioned = PriorityLevelConfigurationReferenceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationReference {
    fn into_optioned(self) -> PriorityLevelConfigurationReferenceAc {
        PriorityLevelConfigurationReferenceAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: PriorityLevelConfigurationReferenceAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: PriorityLevelConfigurationReferenceAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationReference,
> for PriorityLevelConfigurationReferenceAc {
    fn from_optionable(
        value: k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationReference,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationReference,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::flowcontrol::v1beta3::PriorityLevelConfigurationReference,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
