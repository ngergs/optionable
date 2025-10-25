pub struct DownwardAPIVolumeSourceOpt {
    pub default_mode: <Option<i32> as crate::Optionable>::Optioned,
    pub items: <Option<
        std::vec::Vec<::k8s_openapi::api::core::v1::DownwardAPIVolumeFile>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::DownwardAPIVolumeSource {
    type Optioned = DownwardAPIVolumeSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for DownwardAPIVolumeSourceOpt {
    type Optioned = DownwardAPIVolumeSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::DownwardAPIVolumeSource {
    fn into_optioned(self) -> DownwardAPIVolumeSourceOpt {
        DownwardAPIVolumeSourceOpt {
            default_mode: <Option<
                i32,
            > as crate::OptionableConvert>::into_optioned(self.default_mode),
            items: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::DownwardAPIVolumeFile>,
            > as crate::OptionableConvert>::into_optioned(self.items),
        }
    }
    fn try_from_optioned(
        value: DownwardAPIVolumeSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            default_mode: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.default_mode)?,
            items: <Option<
                std::vec::Vec<::k8s_openapi::api::core::v1::DownwardAPIVolumeFile>,
            > as crate::OptionableConvert>::try_from_optioned(value.items)?,
        })
    }
    fn merge(
        &mut self,
        other: DownwardAPIVolumeSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(
            &mut self.default_mode,
            other.default_mode,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::core::v1::DownwardAPIVolumeFile>,
        > as crate::OptionableConvert>::merge(&mut self.items, other.items)?;
        Ok(())
    }
}
