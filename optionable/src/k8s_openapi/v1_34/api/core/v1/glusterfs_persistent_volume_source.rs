pub struct GlusterfsPersistentVolumeSourceAc {
    pub endpoints: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub endpoints_namespace: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    pub path: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub read_only: <Option<bool> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::GlusterfsPersistentVolumeSource {
    type Optioned = GlusterfsPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for GlusterfsPersistentVolumeSourceAc {
    type Optioned = GlusterfsPersistentVolumeSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::GlusterfsPersistentVolumeSource {
    fn into_optioned(self) -> GlusterfsPersistentVolumeSourceAc {
        GlusterfsPersistentVolumeSourceAc {
            endpoints: Some(crate::OptionableConvert::into_optioned(self.endpoints)),
            endpoints_namespace: crate::OptionableConvert::into_optioned(
                self.endpoints_namespace,
            ),
            path: Some(crate::OptionableConvert::into_optioned(self.path)),
            read_only: crate::OptionableConvert::into_optioned(self.read_only),
        }
    }
    fn try_from_optioned(
        value: GlusterfsPersistentVolumeSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            endpoints: crate::OptionableConvert::try_from_optioned(
                value
                    .endpoints
                    .ok_or(crate::optionable::Error {
                        missing_field: "endpoints",
                    })?,
            )?,
            endpoints_namespace: crate::OptionableConvert::try_from_optioned(
                value.endpoints_namespace,
            )?,
            path: crate::OptionableConvert::try_from_optioned(
                value
                    .path
                    .ok_or(crate::optionable::Error {
                        missing_field: "path",
                    })?,
            )?,
            read_only: crate::OptionableConvert::try_from_optioned(value.read_only)?,
        })
    }
    fn merge(
        &mut self,
        other: GlusterfsPersistentVolumeSourceAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.endpoints {
            crate::OptionableConvert::merge(&mut self.endpoints, other_value)?;
        }
        crate::OptionableConvert::merge(
            &mut self.endpoints_namespace,
            other.endpoints_namespace,
        )?;
        if let Some(other_value) = other.path {
            crate::OptionableConvert::merge(&mut self.path, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.read_only, other.read_only)?;
        Ok(())
    }
}
