pub struct VolumeMountOpt {
    pub mount_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub mount_propagation: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    pub recursive_read_only: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub sub_path: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub sub_path_expr: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::volume_mount::VolumeMount {
    type Optioned = VolumeMountOpt;
}
#[automatically_derived]
impl crate::Optionable for VolumeMountOpt {
    type Optioned = VolumeMountOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::volume_mount::VolumeMount {
    fn into_optioned(self) -> VolumeMountOpt {
        VolumeMountOpt {
            mount_path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.mount_path,
                ),
            ),
            mount_propagation: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.mount_propagation),
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
            sub_path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.sub_path),
            sub_path_expr: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.sub_path_expr),
        }
    }
    fn try_from_optioned(
        value: VolumeMountOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            mount_path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .mount_path
                    .ok_or(crate::optionable::Error {
                        missing_field: "mount_path",
                    })?,
            )?,
            mount_propagation: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.mount_propagation)?,
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
            > as crate::OptionableConvert>::try_from_optioned(
                value.recursive_read_only,
            )?,
            sub_path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.sub_path)?,
            sub_path_expr: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.sub_path_expr)?,
        })
    }
    fn merge(&mut self, other: VolumeMountOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.mount_path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.mount_path,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.mount_propagation,
            other.mount_propagation,
        )?;
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
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.sub_path, other.sub_path)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.sub_path_expr,
            other.sub_path_expr,
        )?;
        Ok(())
    }
}
