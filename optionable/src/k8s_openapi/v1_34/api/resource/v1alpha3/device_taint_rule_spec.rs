#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DeviceTaintRuleSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_selector: <Option<
        ::k8s_openapi::api::resource::v1alpha3::DeviceTaintSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taint: Option<
        <::k8s_openapi::api::resource::v1alpha3::DeviceTaint as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::DeviceTaintRuleSpec {
    type Optioned = DeviceTaintRuleSpecAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintRuleSpecAc {
    type Optioned = DeviceTaintRuleSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::DeviceTaintRuleSpec {
    fn into_optioned(self) -> DeviceTaintRuleSpecAc {
        DeviceTaintRuleSpecAc {
            device_selector: crate::OptionableConvert::into_optioned(
                self.device_selector,
            ),
            taint: Some(crate::OptionableConvert::into_optioned(self.taint)),
        }
    }
    fn try_from_optioned(
        value: DeviceTaintRuleSpecAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            device_selector: crate::OptionableConvert::try_from_optioned(
                value.device_selector,
            )?,
            taint: crate::OptionableConvert::try_from_optioned(
                value
                    .taint
                    .ok_or(crate::optionable::Error {
                        missing_field: "taint",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: DeviceTaintRuleSpecAc,
    ) -> Result<(), crate::optionable::Error> {
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
