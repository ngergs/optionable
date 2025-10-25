pub struct ServiceReferenceOpt {
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub port: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference {
    type Optioned = ServiceReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for ServiceReferenceOpt {
    type Optioned = ServiceReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference {
    fn into_optioned(self) -> ServiceReferenceOpt {
        ServiceReferenceOpt {
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.name),
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.namespace),
            port: <Option<i32> as crate::OptionableConvert>::into_optioned(self.port),
        }
    }
    fn try_from_optioned(
        value: ServiceReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.name)?,
            namespace: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.namespace)?,
            port: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.port)?,
        })
    }
    fn merge(
        &mut self,
        other: ServiceReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.name, other.name)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.namespace, other.namespace)?;
        <Option<i32> as crate::OptionableConvert>::merge(&mut self.port, other.port)?;
        Ok(())
    }
}
