pub struct CELDeviceSelectorAc {
    pub expression: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector {
    type Optioned = CELDeviceSelectorAc;
}
#[automatically_derived]
impl crate::Optionable for CELDeviceSelectorAc {
    type Optioned = CELDeviceSelectorAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1alpha3::CELDeviceSelector {
    fn into_optioned(self) -> CELDeviceSelectorAc {
        CELDeviceSelectorAc {
            expression: Some(crate::OptionableConvert::into_optioned(self.expression)),
        }
    }
    fn try_from_optioned(
        value: CELDeviceSelectorAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            expression: crate::OptionableConvert::try_from_optioned(
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
        other: CELDeviceSelectorAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.expression {
            crate::OptionableConvert::merge(&mut self.expression, other_value)?;
        }
        Ok(())
    }
}
