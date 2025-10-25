pub struct DownwardAPIVolumeFileOpt {
    pub field_ref: <Option<
        ::k8s_openapi::api::core::v1::ObjectFieldSelector,
    > as crate::Optionable>::Optioned,
    pub mode: <Option<i32> as crate::Optionable>::Optioned,
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub resource_field_ref: <Option<
        ::k8s_openapi::api::core::v1::ResourceFieldSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::DownwardAPIVolumeFile {
    type Optioned = DownwardAPIVolumeFileOpt;
}
#[automatically_derived]
impl crate::Optionable for DownwardAPIVolumeFileOpt {
    type Optioned = DownwardAPIVolumeFileOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::DownwardAPIVolumeFile {
    fn into_optioned(self) -> DownwardAPIVolumeFileOpt {
        DownwardAPIVolumeFileOpt {
            field_ref: <Option<
                ::k8s_openapi::api::core::v1::ObjectFieldSelector,
            > as crate::OptionableConvert>::into_optioned(self.field_ref),
            mode: <Option<i32> as crate::OptionableConvert>::into_optioned(self.mode),
            path: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.path,
                ),
            ),
            resource_field_ref: <Option<
                ::k8s_openapi::api::core::v1::ResourceFieldSelector,
            > as crate::OptionableConvert>::into_optioned(self.resource_field_ref),
        }
    }
    fn try_from_optioned(
        value: DownwardAPIVolumeFileOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            field_ref: <Option<
                ::k8s_openapi::api::core::v1::ObjectFieldSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.field_ref)?,
            mode: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.mode)?,
            path: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
            resource_field_ref: <Option<
                ::k8s_openapi::api::core::v1::ResourceFieldSelector,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_field_ref)?,
        })
    }
    fn merge(
        &mut self,
        other: DownwardAPIVolumeFileOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::ObjectFieldSelector,
        > as crate::OptionableConvert>::merge(&mut self.field_ref, other.field_ref)?;
        <Option<i32> as crate::OptionableConvert>::merge(&mut self.mode, other.mode)?;
        if let Some(other_value) = other.path {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.path,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::core::v1::ResourceFieldSelector,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_field_ref,
            other.resource_field_ref,
        )?;
        Ok(())
    }
}
