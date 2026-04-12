#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// APIServiceSpec contains information for locating and communicating with a server. Only https is supported, though you are able to disable certificate verification.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct APIServiceSpecAc {
    /// CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate. If unspecified, system trust roots on the apiserver are used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_bundle: Option<<::k8s_openapi027::ByteString as crate::Optionable>::Optioned>,
    /// Group is the API group name this server hosts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<std::string::String>,
    /// GroupPriorityMinimum is the priority this group should have at least. Higher priority means that the group is preferred by clients over lower priority ones. Note that other versions of this group might specify even higher GroupPriorityMinimum values such that the whole group gets a higher priority. The primary sort is based on GroupPriorityMinimum, ordered highest number to lowest (20 before 10). The secondary sort is based on the alphabetical comparison of the name of the object.  (v1.bar before v1.foo) We'd recommend something like: *.k8s.io (except extensions) at 18000 and PaaSes (OpenShift, Deis) are recommended to be in the 2000s
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_priority_minimum: Option<i32>,
    /// InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server. This is strongly discouraged.  You should use the CABundle instead.
    #[serde(rename = "insecureSkipTLSVerify")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_skip_tls_verify: Option<bool>,
    /// Service is a reference to the service for this API server.  It must communicate on port 443. If the Service is nil, that means the handling for the API groupversion is handled locally on this server. The call will simply delegate to the normal handler chain to be fulfilled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<
        <::k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference as crate::Optionable>::Optioned,
    >,
    /// Version is the API version this server hosts.  For example, "v1"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<std::string::String>,
    /// VersionPriority controls the ordering of this API version inside of its group.  Must be greater than zero. The primary sort is based on VersionPriority, ordered highest to lowest (20 before 10). Since it's inside of a group, the number can be small, probably in the 10s. In case of equal version priorities, the version string will be used to compute the order inside a group. If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version), then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first by GA \> beta \> alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing major version, then minor version. An example sorted list of versions: v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_priority: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceSpec {
    type Optioned = APIServiceSpecAc;
}
#[automatically_derived]
impl crate::Optionable for APIServiceSpecAc {
    type Optioned = APIServiceSpecAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceSpec {
    fn into_optioned(self) -> APIServiceSpecAc {
        APIServiceSpecAc {
            ca_bundle: crate::OptionableConvert::into_optioned(self.ca_bundle),
            group: self.group,
            group_priority_minimum: Some(self.group_priority_minimum),
            insecure_skip_tls_verify: self.insecure_skip_tls_verify,
            service: crate::OptionableConvert::into_optioned(self.service),
            version: self.version,
            version_priority: Some(self.version_priority),
        }
    }
    fn try_from_optioned(value: APIServiceSpecAc) -> Result<Self, crate::Error> {
        Ok(Self {
            ca_bundle: crate::OptionableConvert::try_from_optioned(value.ca_bundle)?,
            group: value.group,
            group_priority_minimum: value
                .group_priority_minimum
                .ok_or(crate::Error {
                    missing_field: "group_priority_minimum",
                })?,
            insecure_skip_tls_verify: value.insecure_skip_tls_verify,
            service: crate::OptionableConvert::try_from_optioned(value.service)?,
            version: value.version,
            version_priority: value
                .version_priority
                .ok_or(crate::Error {
                    missing_field: "version_priority",
                })?,
        })
    }
    fn merge(&mut self, other: APIServiceSpecAc) -> Result<(), crate::Error> {
        if self.ca_bundle.is_none() {
            self.ca_bundle = other.ca_bundle;
        }
        if let Some(other_value) = other.ca_bundle {
            crate::OptionableConvert::merge(&mut self.ca_bundle, other_value)?;
        }
        if self.group.is_none() {
            self.group = other.group;
        }
        if let Some(other_value) = other.group {
            crate::OptionableConvert::merge(&mut self.group, other_value)?;
        }
        if let Some(other_value) = other.group_priority_minimum {
            self.group_priority_minimum = other_value;
        }
        if self.insecure_skip_tls_verify.is_none() {
            self.insecure_skip_tls_verify = other.insecure_skip_tls_verify;
        }
        if let Some(other_value) = other.insecure_skip_tls_verify {
            crate::OptionableConvert::merge(
                &mut self.insecure_skip_tls_verify,
                other_value,
            )?;
        }
        if self.service.is_none() {
            self.service = other.service;
        }
        if let Some(other_value) = other.service {
            crate::OptionableConvert::merge(&mut self.service, other_value)?;
        }
        if self.version.is_none() {
            self.version = other.version;
        }
        if let Some(other_value) = other.version {
            crate::OptionableConvert::merge(&mut self.version, other_value)?;
        }
        if let Some(other_value) = other.version_priority {
            self.version_priority = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceSpec,
> for APIServiceSpecAc {
    fn from_optionable(
        value: k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceSpec,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceSpec,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceSpec,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
