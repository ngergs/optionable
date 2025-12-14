#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct VolumeMountStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[cfg(feature = "k8s_openapi_convert")]
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
    fn try_from_optioned(value: VolumeMountStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            mount_path: crate::OptionableConvert::try_from_optioned(
                value
                    .mount_path
                    .ok_or(crate::Error {
                        missing_field: "mount_path",
                    })?,
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
        })
    }
    fn merge(&mut self, other: VolumeMountStatusAc) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::VolumeMountStatus>
for VolumeMountStatusAc {
    fn from_optionable(value: ::k8s_openapi::api::core::v1::VolumeMountStatus) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::VolumeMountStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::VolumeMountStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
