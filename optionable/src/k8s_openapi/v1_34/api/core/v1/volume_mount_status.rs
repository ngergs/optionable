pub struct VolumeMountStatusAc {
    pub mount_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub recursive_read_only: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::VolumeMountStatus {
    type Optioned = VolumeMountStatusAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeMountStatusAc {
    type Optioned = VolumeMountStatusAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::VolumeMountStatus {
    fn into_optioned(self) -> VolumeMountStatusAc {
        VolumeMountStatusAc {
            mount_path: Some(crate::OptionableConvert::into_optioned(self.mount_path)),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            recursive_read_only: crate::OptionableConvert::into_optioned(
                self.recursive_read_only,
            ),
        }
    }
    fn try_from_optioned(
        value: VolumeMountStatusAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            mount_path: crate::OptionableConvert::try_from_optioned(
                value
                    .mount_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "mount_path",
                    })?,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            recursive_read_only: crate::OptionableConvert::try_from_optioned(
                value.recursive_read_only,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeMountStatusAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.mount_path {
            crate::OptionableConvert::merge(&mut self.mount_path, other_value)?;
        }
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(
            &mut self.recursive_read_only,
            other.recursive_read_only,
        )?;
        Ok(())
    }
}
