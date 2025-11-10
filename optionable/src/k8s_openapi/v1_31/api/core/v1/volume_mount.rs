#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMountAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_propagation: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive_read_only: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path_expr: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::VolumeMount {
    type Optioned = VolumeMountAc;
}
#[automatically_derived]
impl crate::Optionable for VolumeMountAc {
    type Optioned = VolumeMountAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::VolumeMount {
    fn into_optioned(self) -> VolumeMountAc {
        VolumeMountAc {
            mount_path: Some(crate::OptionableConvert::into_optioned(self.mount_path)),
            mount_propagation: crate::OptionableConvert::into_optioned(
                self.mount_propagation,
            ),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
            recursive_read_only: crate::OptionableConvert::into_optioned(
                self.recursive_read_only,
            ),
            sub_path: crate::OptionableConvert::into_optioned(self.sub_path),
            sub_path_expr: crate::OptionableConvert::into_optioned(self.sub_path_expr),
        }
    }
    fn try_from_optioned(value: VolumeMountAc) -> Result<Self, crate::Error> {
        Ok(Self {
            mount_path: crate::OptionableConvert::try_from_optioned(
                value
                    .mount_path
                    .ok_or(crate::Error {
                        missing_field: "mount_path",
                    })?,
            )?,
            mount_propagation: crate::OptionableConvert::try_from_optioned(
                value.mount_propagation,
            )?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::Error {
                        missing_field: "name",
                    })?,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
            recursive_read_only: crate::OptionableConvert::try_from_optioned(
                value.recursive_read_only,
            )?,
            sub_path: crate::OptionableConvert::try_from_optioned(value.sub_path)?,
            sub_path_expr: crate::OptionableConvert::try_from_optioned(
                value.sub_path_expr,
            )?,
        })
    }
    fn merge(&mut self, other: VolumeMountAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.mount_path {
            crate::OptionableConvert::merge(&mut self.mount_path, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.mount_propagation,
            other.mount_propagation,
        )?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        crate::OptionableConvert::merge(
            &mut self.recursive_read_only,
            other.recursive_read_only,
        )?;
        crate::OptionableConvert::merge(&mut self.sub_path, other.sub_path)?;
        crate::OptionableConvert::merge(&mut self.sub_path_expr, other.sub_path_expr)?;
        Ok(())
    }
}
