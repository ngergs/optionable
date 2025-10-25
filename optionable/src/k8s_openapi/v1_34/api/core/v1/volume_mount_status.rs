pub struct VolumeMountStatusOpt {
    pub mount_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub recursive_read_only: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::VolumeMountStatus {
    type Optioned = VolumeMountStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeMountStatusOpt {
    type Optioned = VolumeMountStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::VolumeMountStatus {
    fn into_optioned(self) -> VolumeMountStatusOpt {
        VolumeMountStatusOpt {
            mount_path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.mount_path,
                ),
            ),
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.read_only),
            recursive_read_only: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.recursive_read_only),
        }
    }
    fn try_from_optioned(
        value: VolumeMountStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            mount_path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .mount_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "mount_path",
                    })?,
            )?,
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            read_only: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.read_only)?,
            recursive_read_only: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.recursive_read_only)?,
        })
    }
    fn merge(
        &mut self,
        other: VolumeMountStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.mount_path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.mount_path,
                other_value,
            )?;
        }
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.read_only, other.read_only)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.recursive_read_only,
            other.recursive_read_only,
        )?;
        Ok(())
    }
}
