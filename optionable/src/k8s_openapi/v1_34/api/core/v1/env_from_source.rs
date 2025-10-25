pub struct EnvFromSourceOpt {
    pub config_map_ref: <Option<
        ::k8s_openapi::api::core::v1::ConfigMapEnvSource,
    > as crate::Optionable>::Optioned,
    pub prefix: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub secret_ref: <Option<
        ::k8s_openapi::api::core::v1::SecretEnvSource,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::EnvFromSource {
    type Optioned = EnvFromSourceOpt;
}
#[automatically_derived]
impl crate::Optionable for EnvFromSourceOpt {
    type Optioned = EnvFromSourceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::EnvFromSource {
    fn into_optioned(self) -> EnvFromSourceOpt {
        EnvFromSourceOpt {
            config_map_ref: <Option<
                ::k8s_openapi::api::core::v1::ConfigMapEnvSource,
            > as crate::OptionableConvert>::into_optioned(self.config_map_ref),
            prefix: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.prefix),
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretEnvSource,
            > as crate::OptionableConvert>::into_optioned(self.secret_ref),
        }
    }
    fn try_from_optioned(
        value: EnvFromSourceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            config_map_ref: <Option<
                ::k8s_openapi::api::core::v1::ConfigMapEnvSource,
            > as crate::OptionableConvert>::try_from_optioned(value.config_map_ref)?,
            prefix: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.prefix)?,
            secret_ref: <Option<
                ::k8s_openapi::api::core::v1::SecretEnvSource,
            > as crate::OptionableConvert>::try_from_optioned(value.secret_ref)?,
        })
    }
    fn merge(
        &mut self,
        other: EnvFromSourceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::core::v1::ConfigMapEnvSource,
        > as crate::OptionableConvert>::merge(
            &mut self.config_map_ref,
            other.config_map_ref,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.prefix, other.prefix)?;
        <Option<
            ::k8s_openapi::api::core::v1::SecretEnvSource,
        > as crate::OptionableConvert>::merge(&mut self.secret_ref, other.secret_ref)?;
        Ok(())
    }
}
