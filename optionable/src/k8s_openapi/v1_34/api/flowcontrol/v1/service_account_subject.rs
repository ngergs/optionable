pub struct ServiceAccountSubjectOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub namespace: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject {
    type Optioned = ServiceAccountSubjectOpt;
}
#[automatically_derived]
impl crate::Optionable for ServiceAccountSubjectOpt {
    type Optioned = ServiceAccountSubjectOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::ServiceAccountSubject {
    fn into_optioned(self) -> ServiceAccountSubjectOpt {
        ServiceAccountSubjectOpt {
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
        }
    }
    fn try_from_optioned(
        value: ServiceAccountSubjectOpt,
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
        })
    }
    fn merge(
        &mut self,
        other: ServiceAccountSubjectOpt,
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
        Ok(())
    }
}
