pub struct IngressSpecOpt {
    pub default_backend: <Option<
        ::k8s_openapi::api::networking::v1::IngressBackend,
    > as crate::Optionable>::Optioned,
    pub ingress_class_name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub rules: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::IngressRule>,
    > as crate::Optionable>::Optioned,
    pub tls: <Option<
        std::vec::Vec<::k8s_openapi::api::networking::v1::IngressTLS>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressSpec {
    type Optioned = IngressSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressSpecOpt {
    type Optioned = IngressSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::IngressSpec {
    fn into_optioned(self) -> IngressSpecOpt {
        IngressSpecOpt {
            default_backend: <Option<
                ::k8s_openapi::api::networking::v1::IngressBackend,
            > as crate::OptionableConvert>::into_optioned(self.default_backend),
            ingress_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.ingress_class_name),
            rules: <Option<
                std::vec::Vec<::k8s_openapi::api::networking::v1::IngressRule>,
            > as crate::OptionableConvert>::into_optioned(self.rules),
            tls: <Option<
                std::vec::Vec<::k8s_openapi::api::networking::v1::IngressTLS>,
            > as crate::OptionableConvert>::into_optioned(self.tls),
        }
    }
    fn try_from_optioned(
        value: IngressSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            default_backend: <Option<
                ::k8s_openapi::api::networking::v1::IngressBackend,
            > as crate::OptionableConvert>::try_from_optioned(value.default_backend)?,
            ingress_class_name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.ingress_class_name)?,
            rules: <Option<
                std::vec::Vec<::k8s_openapi::api::networking::v1::IngressRule>,
            > as crate::OptionableConvert>::try_from_optioned(value.rules)?,
            tls: <Option<
                std::vec::Vec<::k8s_openapi::api::networking::v1::IngressTLS>,
            > as crate::OptionableConvert>::try_from_optioned(value.tls)?,
        })
    }
    fn merge(&mut self, other: IngressSpecOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::api::networking::v1::IngressBackend,
        > as crate::OptionableConvert>::merge(
            &mut self.default_backend,
            other.default_backend,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.ingress_class_name,
            other.ingress_class_name,
        )?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::networking::v1::IngressRule>,
        > as crate::OptionableConvert>::merge(&mut self.rules, other.rules)?;
        <Option<
            std::vec::Vec<::k8s_openapi::api::networking::v1::IngressTLS>,
        > as crate::OptionableConvert>::merge(&mut self.tls, other.tls)?;
        Ok(())
    }
}
