#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceTaintAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_added: <Option<
        ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::DeviceTaint {
    type Optioned = DeviceTaintAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintAc {
    type Optioned = DeviceTaintAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1alpha3::DeviceTaint {
    fn into_optioned(self) -> DeviceTaintAc {
        DeviceTaintAc {
            effect: Some(crate::OptionableConvert::into_optioned(self.effect)),
            key: Some(crate::OptionableConvert::into_optioned(self.key)),
            time_added: crate::OptionableConvert::into_optioned(self.time_added),
            value: crate::OptionableConvert::into_optioned(self.value),
        }
    }
    fn try_from_optioned(value: DeviceTaintAc) -> Result<Self, crate::Error> {
        Ok(Self {
            effect: crate::OptionableConvert::try_from_optioned(
                value
                    .effect
                    .ok_or(crate::Error {
                        missing_field: "effect",
                    })?,
            )?,
            key: crate::OptionableConvert::try_from_optioned(
                value
                    .key
                    .ok_or(crate::Error {
                        missing_field: "key",
                    })?,
            )?,
            time_added: crate::OptionableConvert::try_from_optioned(value.time_added)?,
            value: crate::OptionableConvert::try_from_optioned(value.value)?,
        })
    }
    fn merge(&mut self, other: DeviceTaintAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.effect {
            crate::OptionableConvert::merge(&mut self.effect, other_value)?;
        }
        if let Some(other_value) = other.key {
            crate::OptionableConvert::merge(&mut self.key, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.time_added, other.time_added)?;
        crate::OptionableConvert::merge(&mut self.value, other.value)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::DeviceTaint>
for DeviceTaintAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceTaint,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1alpha3::DeviceTaint, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceTaint,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
