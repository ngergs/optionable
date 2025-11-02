#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyVolumeStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_volume_attributes_class_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ModifyVolumeStatus {
    type Optioned = ModifyVolumeStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ModifyVolumeStatusAc {
    type Optioned = ModifyVolumeStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ModifyVolumeStatus {
    fn into_optioned(self) -> ModifyVolumeStatusAc {
        ModifyVolumeStatusAc {
            status: Some(crate::OptionableConvert::into_optioned(self.status)),
            target_volume_attributes_class_name: crate::OptionableConvert::into_optioned(
                self.target_volume_attributes_class_name,
            ),
        }
    }
    fn try_from_optioned(
        value: ModifyVolumeStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            status: crate::OptionableConvert::try_from_optioned(
                value
                    .status
                    .ok_or(crate::optionable::Error {
                        missing_field: "status",
                    })?,
            )?,
            target_volume_attributes_class_name: crate::OptionableConvert::try_from_optioned(
                value.target_volume_attributes_class_name,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ModifyVolumeStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.status {
            crate::OptionableConvert::merge(&mut self.status, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.target_volume_attributes_class_name,
            other.target_volume_attributes_class_name,
        )?;
        Ok(())
    }
}
