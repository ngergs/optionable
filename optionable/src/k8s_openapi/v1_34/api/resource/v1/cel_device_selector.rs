pub struct CELDeviceSelectorOpt {
    pub expression: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1::cel_device_selector::CELDeviceSelector {
    type Optioned = CELDeviceSelectorOpt;
}
#[automatically_derived]
impl crate::Optionable for CELDeviceSelectorOpt {
    type Optioned = CELDeviceSelectorOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1::cel_device_selector::CELDeviceSelector {
    fn into_optioned(self) -> CELDeviceSelectorOpt {
        CELDeviceSelectorOpt {
            expression: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.expression,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: CELDeviceSelectorOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .expression
                    .ok_or(crate::optionable::Error {
                        missing_field: "expression",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CELDeviceSelectorOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.expression {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.expression,
                other_value,
            )?;
        }
        Ok(())
    }
}
