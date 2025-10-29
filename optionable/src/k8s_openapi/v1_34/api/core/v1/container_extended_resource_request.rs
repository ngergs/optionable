pub struct ContainerExtendedResourceRequestOpt {
    pub container_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub request_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub resource_name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::core::v1::ContainerExtendedResourceRequest {
    type Optioned = ContainerExtendedResourceRequestOpt;
}
#[automatically_derived]
impl crate::Optionable for ContainerExtendedResourceRequestOpt {
    type Optioned = ContainerExtendedResourceRequestOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::core::v1::ContainerExtendedResourceRequest {
    fn into_optioned(self) -> ContainerExtendedResourceRequestOpt {
        ContainerExtendedResourceRequestOpt {
            container_name: Some(
                crate::OptionableConvert::into_optioned(self.container_name),
            ),
            request_name: Some(
                crate::OptionableConvert::into_optioned(self.request_name),
            ),
            resource_name: Some(
                crate::OptionableConvert::into_optioned(self.resource_name),
            ),
        }
    }
    fn try_from_optioned(
        value: ContainerExtendedResourceRequestOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            container_name: crate::OptionableConvert::try_from_optioned(
                value
                    .container_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "container_name",
                    })?,
            )?,
            request_name: crate::OptionableConvert::try_from_optioned(
                value
                    .request_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "request_name",
                    })?,
            )?,
            resource_name: crate::OptionableConvert::try_from_optioned(
                value
                    .resource_name
                    .ok_or(crate::optionable::Error {
                        missing_field: "resource_name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerExtendedResourceRequestOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.container_name {
            crate::OptionableConvert::merge(&mut self.container_name, other_value)?;
        }
        if let Some(other_value) = other.request_name {
            crate::OptionableConvert::merge(&mut self.request_name, other_value)?;
        }
        if let Some(other_value) = other.resource_name {
            crate::OptionableConvert::merge(&mut self.resource_name, other_value)?;
        }
        Ok(())
    }
}
