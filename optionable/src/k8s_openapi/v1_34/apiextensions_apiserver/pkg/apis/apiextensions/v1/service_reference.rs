pub struct ServiceReferenceOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub namespace: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub port: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference {
    type Optioned = ServiceReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for ServiceReferenceOpt {
    type Optioned = ServiceReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::ServiceReference {
    fn into_optioned(self) -> ServiceReferenceOpt {
        ServiceReferenceOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
            namespace: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.namespace,
                ),
            ),
            path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.path),
            port: <Option<i32> as crate::OptionableConvert>::into_optioned(self.port),
        }
    }
    fn try_from_optioned(
        value: ServiceReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            namespace: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .namespace
                    .ok_or(crate::optionable::Error {
                        missing_field: "namespace",
                    })?,
            )?,
            path: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.path)?,
            port: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.port)?,
        })
    }
    fn merge(
        &mut self,
        other: ServiceReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.namespace {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.namespace,
                other_value,
            )?;
        }
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.path, other.path)?;
        <Option<i32> as crate::OptionableConvert>::merge(&mut self.port, other.port)?;
        Ok(())
    }
}
