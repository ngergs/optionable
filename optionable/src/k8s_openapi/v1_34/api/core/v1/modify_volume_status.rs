pub struct ModifyVolumeStatusOpt {
    pub status: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub target_volume_attributes_class_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::modify_volume_status::ModifyVolumeStatus {
    type Optioned = ModifyVolumeStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for ModifyVolumeStatusOpt {
    type Optioned = ModifyVolumeStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::modify_volume_status::ModifyVolumeStatus {
    fn into_optioned(self) -> ModifyVolumeStatusOpt {
        ModifyVolumeStatusOpt {
            status: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.status,
                ),
            ),
            target_volume_attributes_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(
                self.target_volume_attributes_class_name,
            ),
        }
    }
    fn try_from_optioned(
        value: ModifyVolumeStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            status: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .status
                    .ok_or(crate::optionable::Error {
                        missing_field: "status",
                    })?,
            )?,
            target_volume_attributes_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(
                value.target_volume_attributes_class_name,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ModifyVolumeStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.status {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.status,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.target_volume_attributes_class_name,
            other.target_volume_attributes_class_name,
        )?;
        Ok(())
    }
}
