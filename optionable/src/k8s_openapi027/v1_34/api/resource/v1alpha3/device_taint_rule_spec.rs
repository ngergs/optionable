#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceTaintRuleSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_selector: <Option<
        ::k8s_openapi027::api::resource::v1alpha3::DeviceTaintSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taint: Option<
        <::k8s_openapi027::api::resource::v1alpha3::DeviceTaint as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleSpec {
    type Optioned = DeviceTaintRuleSpecAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintRuleSpecAc {
    type Optioned = DeviceTaintRuleSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleSpec {
    fn into_optioned(self) -> DeviceTaintRuleSpecAc {
        DeviceTaintRuleSpecAc {
            device_selector: crate::OptionableConvert::into_optioned(
                self.device_selector,
            ),
            taint: Some(crate::OptionableConvert::into_optioned(self.taint)),
        }
    }
    fn try_from_optioned(value: DeviceTaintRuleSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            device_selector: crate::OptionableConvert::try_from_optioned(
                value.device_selector,
            )?,
            taint: crate::OptionableConvert::try_from_optioned(
                value
                    .taint
                    .ok_or(crate::Error {
                        missing_field: "taint",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: DeviceTaintRuleSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.device_selector,
            other.device_selector,
        )?;
        if let Some(other_value) = other.taint {
            crate::OptionableConvert::merge(&mut self.taint, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleSpec>
for DeviceTaintRuleSpecAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1alpha3::DeviceTaintRuleSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
