pub struct EnvVarSourceAc {
    pub config_map_key_ref: <Option<
        ::k8s_openapi::api::core::v1::ConfigMapKeySelector,
    > as crate::Optionable>::Optioned,
    pub field_ref: <Option<
        ::k8s_openapi::api::core::v1::ObjectFieldSelector,
    > as crate::Optionable>::Optioned,
    pub file_key_ref: <Option<
        ::k8s_openapi::api::core::v1::FileKeySelector,
    > as crate::Optionable>::Optioned,
    pub resource_field_ref: <Option<
        ::k8s_openapi::api::core::v1::ResourceFieldSelector,
    > as crate::Optionable>::Optioned,
    pub secret_key_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretKeySelector,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EnvVarSource {
    type Optioned = EnvVarSourceAc;
}
#[automatically_derived]
impl crate::Optionable for EnvVarSourceAc {
    type Optioned = EnvVarSourceAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EnvVarSource {
    fn into_optioned(self) -> EnvVarSourceAc {
        EnvVarSourceAc {
            config_map_key_ref: crate::OptionableConvert::into_optioned(
                self.config_map_key_ref,
            ),
            field_ref: crate::OptionableConvert::into_optioned(self.field_ref),
            file_key_ref: crate::OptionableConvert::into_optioned(self.file_key_ref),
            resource_field_ref: crate::OptionableConvert::into_optioned(
                self.resource_field_ref,
            ),
            secret_key_ref: crate::OptionableConvert::into_optioned(self.secret_key_ref),
        }
    }
    fn try_from_optioned(
        value: EnvVarSourceAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config_map_key_ref: crate::OptionableConvert::try_from_optioned(
                value.config_map_key_ref,
            )?,
            field_ref: crate::OptionableConvert::try_from_optioned(value.field_ref)?,
            file_key_ref: crate::OptionableConvert::try_from_optioned(
                value.file_key_ref,
            )?,
            resource_field_ref: crate::OptionableConvert::try_from_optioned(
                value.resource_field_ref,
            )?,
            secret_key_ref: crate::OptionableConvert::try_from_optioned(
                value.secret_key_ref,
            )?,
        })
    }
    fn merge(&mut self, other: EnvVarSourceAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.config_map_key_ref,
            other.config_map_key_ref,
        )?;
        crate::OptionableConvert::merge(&mut self.field_ref, other.field_ref)?;
        crate::OptionableConvert::merge(&mut self.file_key_ref, other.file_key_ref)?;
        crate::OptionableConvert::merge(
            &mut self.resource_field_ref,
            other.resource_field_ref,
        )?;
        crate::OptionableConvert::merge(&mut self.secret_key_ref, other.secret_key_ref)?;
        Ok(())
    }
}
