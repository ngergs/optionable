#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GlusterfsVolumeSourceAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::GlusterfsVolumeSource {
    type Optioned = GlusterfsVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for GlusterfsVolumeSourceAc {
    type Optioned = GlusterfsVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::GlusterfsVolumeSource {
    fn into_optioned(self) -> GlusterfsVolumeSourceAc {
        GlusterfsVolumeSourceAc {
            endpoints: Some(crate::OptionableConvert::into_optioned(self.endpoints)),
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
        }
    }
    fn try_from_optioned(value: GlusterfsVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            endpoints: crate::OptionableConvert::try_from_optioned(
                value
                    .endpoints
                    .ok_or(crate::Error {
                        missing_field: "endpoints",
                    })?,
            )?,
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::Error {
                        missing_field: "path",
                    })?,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
        })
    }
    fn merge(&mut self, other: GlusterfsVolumeSourceAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.endpoints {
            crate::OptionableConvert::merge(&mut self.endpoints, other_value)?;
        }
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::GlusterfsVolumeSource>
for GlusterfsVolumeSourceAc {
    fn from_optionable(
        value: k8s_openapi026::api::core::v1::GlusterfsVolumeSource,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::GlusterfsVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::GlusterfsVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
