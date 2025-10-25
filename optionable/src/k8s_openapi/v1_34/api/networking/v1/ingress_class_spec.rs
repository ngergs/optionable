pub struct IngressClassSpecOpt {
    pub controller: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub parameters: <Option<
        ::k8s_openapi::api::networking::v1::IngressClassParametersReference,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::networking::v1::ingress_class_spec::IngressClassSpec {
    type Optioned = IngressClassSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressClassSpecOpt {
    type Optioned = IngressClassSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::ingress_class_spec::IngressClassSpec {
    fn into_optioned(self) -> IngressClassSpecOpt {
        IngressClassSpecOpt {
            controller: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.controller),
            parameters: <Option<
                ::k8s_openapi::api::networking::v1::IngressClassParametersReference,
            > as crate::OptionableConvert>::into_optioned(self.parameters),
        }
    }
    fn try_from_optioned(
        value: IngressClassSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            controller: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.controller)?,
            parameters: <Option<
                ::k8s_openapi::api::networking::v1::IngressClassParametersReference,
            > as crate::OptionableConvert>::try_from_optioned(value.parameters)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressClassSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.controller, other.controller)?;
        <Option<
            ::k8s_openapi::api::networking::v1::IngressClassParametersReference,
        > as crate::OptionableConvert>::merge(&mut self.parameters, other.parameters)?;
        Ok(())
    }
}
