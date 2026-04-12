#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ContainerExtendedResourceRequest has the mapping of container name, extended resource name to the device request name.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContainerExtendedResourceRequestAc {
    /// The name of the container requesting resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<std::string::String>,
    /// The name of the request in the special ResourceClaim which corresponds to the extended resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<std::string::String>,
    /// The name of the extended resource in that container which gets backed by DRA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<std::string::String>,
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
            container_name: Some(self.container_name),
            request_name: Some(self.request_name),
            resource_name: Some(self.resource_name),
        }
    }
    fn try_from_optioned(
        value: ContainerExtendedResourceRequestAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            container_name: value
                .container_name
                .ok_or(crate::Error {
                    missing_field: "container_name",
                })?,
            request_name: value
                .request_name
                .ok_or(crate::Error {
                    missing_field: "request_name",
                })?,
            resource_name: value
                .resource_name
                .ok_or(crate::Error {
                    missing_field: "resource_name",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: ContainerExtendedResourceRequestAc,
    ) -> Result<(), crate::Error> {
        if let Some(other_value) = other.container_name {
            self.container_name = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.request_name {
            self.request_name = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
        }
        if let Some(other_value) = other.resource_name {
            self.resource_name = crate::OptionableConvert::try_from_optioned(
                other_value,
            )?;
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
