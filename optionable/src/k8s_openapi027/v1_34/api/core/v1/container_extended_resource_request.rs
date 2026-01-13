#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerExtendedResourceRequestAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::core::v1::ContainerExtendedResourceRequest {
    type Optioned = ContainerExtendedResourceRequestAc;
}
#[automatically_derived]
impl crate::Optionable for ContainerExtendedResourceRequestAc {
    type Optioned = ContainerExtendedResourceRequestAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::core::v1::ContainerExtendedResourceRequest {
    fn into_optioned(self) -> ContainerExtendedResourceRequestAc {
        ContainerExtendedResourceRequestAc {
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
        value: ContainerExtendedResourceRequestAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            container_name: crate::OptionableConvert::try_from_optioned(
                value
                    .container_name
                    .ok_or(crate::Error {
                        missing_field: "container_name",
                    })?,
            )?,
            request_name: crate::OptionableConvert::try_from_optioned(
                value
                    .request_name
                    .ok_or(crate::Error {
                        missing_field: "request_name",
                    })?,
            )?,
            resource_name: crate::OptionableConvert::try_from_optioned(
                value
                    .resource_name
                    .ok_or(crate::Error {
                        missing_field: "resource_name",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerExtendedResourceRequestAc,
    ) -> Result<(), crate::Error> {
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
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::core::v1::ContainerExtendedResourceRequest,
> for ContainerExtendedResourceRequestAc {
    fn from_optionable(
        value: k8s_openapi027::api::core::v1::ContainerExtendedResourceRequest,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::core::v1::ContainerExtendedResourceRequest,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ContainerExtendedResourceRequest,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
