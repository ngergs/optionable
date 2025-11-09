#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct WindowsSecurityContextOptionsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec_name: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_process: <Option<bool> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_user_name: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::WindowsSecurityContextOptions {
    type Optioned = WindowsSecurityContextOptionsAc;
}
#[automatically_derived]
impl crate::Optionable for WindowsSecurityContextOptionsAc {
    type Optioned = WindowsSecurityContextOptionsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::WindowsSecurityContextOptions {
    fn into_optioned(self) -> WindowsSecurityContextOptionsAc {
        WindowsSecurityContextOptionsAc {
            gmsa_credential_spec: crate::OptionableConvert::into_optioned(
                self.gmsa_credential_spec,
            ),
            gmsa_credential_spec_name: crate::OptionableConvert::into_optioned(
                self.gmsa_credential_spec_name,
            ),
            host_process: crate::OptionableConvert::into_optioned(self.host_process),
            run_as_user_name: crate::OptionableConvert::into_optioned(
                self.run_as_user_name,
            ),
        }
    }
    fn try_from_optioned(
        value: WindowsSecurityContextOptionsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            gmsa_credential_spec: crate::OptionableConvert::try_from_optioned(
                value.gmsa_credential_spec,
            )?,
            gmsa_credential_spec_name: crate::OptionableConvert::try_from_optioned(
                value.gmsa_credential_spec_name,
            )?,
            host_process: crate::OptionableConvert::try_from_optioned(
                value.host_process,
            )?,
            run_as_user_name: crate::OptionableConvert::try_from_optioned(
                value.run_as_user_name,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: WindowsSecurityContextOptionsAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.gmsa_credential_spec,
            other.gmsa_credential_spec,
        )?;
        crate::OptionableConvert::merge(
            &mut self.gmsa_credential_spec_name,
            other.gmsa_credential_spec_name,
        )?;
        crate::OptionableConvert::merge(&mut self.host_process, other.host_process)?;
        crate::OptionableConvert::merge(
            &mut self.run_as_user_name,
            other.run_as_user_name,
        )?;
        Ok(())
    }
}
