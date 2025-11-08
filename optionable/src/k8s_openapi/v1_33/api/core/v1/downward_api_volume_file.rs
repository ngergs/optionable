#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct DownwardAPIVolumeFileAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_ref: <Option<
        ::k8s_openapi::api::core::v1::ObjectFieldSelector,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_field_ref: <Option<
        ::k8s_openapi::api::core::v1::ResourceFieldSelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::DownwardAPIVolumeFile {
    type Optioned = DownwardAPIVolumeFileAc;
}
#[automatically_derived]
impl crate::Optionable for DownwardAPIVolumeFileAc {
    type Optioned = DownwardAPIVolumeFileAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::DownwardAPIVolumeFile {
    fn into_optioned(self) -> DownwardAPIVolumeFileAc {
        DownwardAPIVolumeFileAc {
            field_ref: crate::OptionableConvert::into_optioned(self.field_ref),
            mode: crate::OptionableConvert::into_optioned(self.mode),
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
            resource_field_ref: crate::OptionableConvert::into_optioned(
                self.resource_field_ref,
            ),
        }
    }
    fn try_from_optioned(value: DownwardAPIVolumeFileAc) -> Result<Self, crate::Error> {
        Ok(Self {
            field_ref: crate::OptionableConvert::try_from_optioned(value.field_ref)?,
            mode: crate::OptionableConvert::try_from_optioned(value.mode)?,
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::Error {
                        missing_field: "path",
                    })?,
            )?,
            resource_field_ref: crate::OptionableConvert::try_from_optioned(
                value.resource_field_ref,
            )?,
        })
    }
    fn merge(&mut self, other: DownwardAPIVolumeFileAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.field_ref, other.field_ref)?;
        crate::OptionableConvert::merge(&mut self.mode, other.mode)?;
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.resource_field_ref,
            other.resource_field_ref,
        )?;
        Ok(())
    }
}
