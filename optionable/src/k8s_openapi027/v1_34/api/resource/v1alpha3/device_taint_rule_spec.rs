#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DeviceTaintRuleSpec specifies the selector and one taint.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceTaintRuleSpecAc {
    /// DeviceSelector defines which device(s) the taint is applied to. All selector criteria must be satified for a device to match. The empty selector matches all devices. Without a selector, no devices are matches.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_selector: Option<
        <::k8s_openapi027::api::resource::v1alpha3::DeviceTaintSelector as crate::Optionable>::Optioned,
    >,
    /// The taint that gets applied to matching devices.
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
        if self.device_selector.is_none() {
            self.device_selector = crate::OptionableConvert::try_from_optioned(
                other.device_selector,
            )?;
        } else if let Some(self_value) = self.device_selector.as_mut()
            && let Some(other_value) = other.device_selector
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
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
