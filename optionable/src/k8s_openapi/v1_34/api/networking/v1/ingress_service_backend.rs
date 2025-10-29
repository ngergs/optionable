pub struct IngressServiceBackendOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub port: <Option<
        ::k8s_openapi::api::networking::v1::ServiceBackendPort,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::IngressServiceBackend {
    type Optioned = IngressServiceBackendOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressServiceBackendOpt {
    type Optioned = IngressServiceBackendOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::IngressServiceBackend {
    fn into_optioned(self) -> IngressServiceBackendOpt {
        IngressServiceBackendOpt {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            port: crate::OptionableConvert::into_optioned(self.port),
        }
    }
    fn try_from_optioned(
        value: IngressServiceBackendOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            port: crate::OptionableConvert::try_from_optioned(value.port)?,
        })
    }
    fn merge(
        &mut self,
        other: IngressServiceBackendOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.port, other.port)?;
        Ok(())
    }
}
