pub struct ServiceBackendPortOpt {
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub number: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::ServiceBackendPort {
    type Optioned = ServiceBackendPortOpt;
}
#[automatically_derived]
impl crate::Optionable for ServiceBackendPortOpt {
    type Optioned = ServiceBackendPortOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::networking::v1::ServiceBackendPort {
    fn into_optioned(self) -> ServiceBackendPortOpt {
        ServiceBackendPortOpt {
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.name),
            number: <Option<i32> as crate::OptionableConvert>::into_optioned(self.number),
        }
    }
    fn try_from_optioned(
        value: ServiceBackendPortOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.name)?,
            number: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.number)?,
        })
    }
    fn merge(
        &mut self,
        other: ServiceBackendPortOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.name, other.name)?;
        <Option<
            i32,
        > as crate::OptionableConvert>::merge(&mut self.number, other.number)?;
        Ok(())
    }
}
