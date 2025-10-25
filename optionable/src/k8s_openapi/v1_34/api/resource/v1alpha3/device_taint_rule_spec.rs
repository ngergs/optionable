pub struct DeviceTaintRuleSpecOpt {
    pub device_selector: <Option<
        ::k8s_openapi::api::resource::v1alpha3::DeviceTaintSelector,
    > as crate::Optionable>::Optioned,
    pub taint: Option<
        <::k8s_openapi::api::resource::v1alpha3::DeviceTaint as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::DeviceTaintRuleSpec {
    type Optioned = DeviceTaintRuleSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintRuleSpecOpt {
    type Optioned = DeviceTaintRuleSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::DeviceTaintRuleSpec {
    fn into_optioned(self) -> DeviceTaintRuleSpecOpt {
        DeviceTaintRuleSpecOpt {
            device_selector: <Option<
                ::k8s_openapi::api::resource::v1alpha3::DeviceTaintSelector,
            > as crate::OptionableConvert>::into_optioned(self.device_selector),
            taint: Some(
                <::k8s_openapi::api::resource::v1alpha3::DeviceTaint as crate::OptionableConvert>::into_optioned(
                    self.taint,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: DeviceTaintRuleSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            device_selector: <Option<
                ::k8s_openapi::api::resource::v1alpha3::DeviceTaintSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.device_selector)?,
            taint: <::k8s_openapi::api::resource::v1alpha3::DeviceTaint as crate::OptionableConvert>::try_from_optioned(
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
        other: DeviceTaintRuleSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::resource::v1alpha3::DeviceTaintSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.device_selector,
            other.device_selector,
        )?;
        if let Some(other_value) = other.taint {
            <::k8s_openapi::api::resource::v1alpha3::DeviceTaint as crate::OptionableConvert>::merge(
                &mut self.taint,
                other_value,
            )?;
        }
        Ok(())
    }
}
