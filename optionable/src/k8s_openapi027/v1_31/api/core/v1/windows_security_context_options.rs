#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// WindowsSecurityContextOptions contain Windows-specific options and credentials.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WindowsSecurityContextOptionsAc {
    /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec: Option<std::string::String>,
    /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmsa_credential_spec_name: Option<std::string::String>,
    /// HostProcess determines if a container should be run as a 'Host Process' container. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers). In addition, if HostProcess is true then HostNetwork must also be set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_process: Option<bool>,
    /// The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_user_name: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::WindowsSecurityContextOptions {
    type Optioned = WindowsSecurityContextOptionsAc;
}
#[automatically_derived]
impl crate::Optionable for WindowsSecurityContextOptionsAc {
    type Optioned = WindowsSecurityContextOptionsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::WindowsSecurityContextOptions {
    fn into_optioned(self) -> WindowsSecurityContextOptionsAc {
        WindowsSecurityContextOptionsAc {
            gmsa_credential_spec: self.gmsa_credential_spec,
            gmsa_credential_spec_name: self.gmsa_credential_spec_name,
            host_process: self.host_process,
            run_as_user_name: self.run_as_user_name,
        }
    }
    fn try_from_optioned(
        value: WindowsSecurityContextOptionsAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            gmsa_credential_spec: value.gmsa_credential_spec,
            gmsa_credential_spec_name: value.gmsa_credential_spec_name,
            host_process: value.host_process,
            run_as_user_name: value.run_as_user_name,
        })
    }
    fn merge(
        &mut self,
        other: WindowsSecurityContextOptionsAc,
    ) -> Result<(), crate::Error> {
        if self.gmsa_credential_spec.is_none() {
            self.gmsa_credential_spec = crate::OptionableConvert::try_from_optioned(
                other.gmsa_credential_spec,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.gmsa_credential_spec,
                other.gmsa_credential_spec,
            )?;
        }
        if self.gmsa_credential_spec_name.is_none() {
            self.gmsa_credential_spec_name = crate::OptionableConvert::try_from_optioned(
                other.gmsa_credential_spec_name,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.gmsa_credential_spec_name,
                other.gmsa_credential_spec_name,
            )?;
        }
        if self.host_process.is_none() {
            self.host_process = crate::OptionableConvert::try_from_optioned(
                other.host_process,
            )?;
        } else {
            crate::OptionableConvert::merge(&mut self.host_process, other.host_process)?;
        }
        if self.run_as_user_name.is_none() {
            self.run_as_user_name = crate::OptionableConvert::try_from_optioned(
                other.run_as_user_name,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.run_as_user_name,
                other.run_as_user_name,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::WindowsSecurityContextOptions>
for WindowsSecurityContextOptionsAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::WindowsSecurityContextOptions,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::WindowsSecurityContextOptions,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::WindowsSecurityContextOptions,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
