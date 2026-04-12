#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// HTTPIngressPath associates a path with a backend. Incoming urls matching the path are forwarded to the backend.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HTTPIngressPathAc {
    /// backend defines the referenced service endpoint to which the traffic will be forwarded to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<
        <::k8s_openapi027::api::networking::v1::IngressBackend as crate::Optionable>::Optioned,
    >,
    /// path is matched against the path of an incoming request. Currently it can contain characters disallowed from the conventional "path" part of a URL as defined by RFC 3986. Paths must begin with a '/' and must be present when using PathType with value "Exact" or "Prefix".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<std::string::String>,
    /// pathType determines the interpretation of the path matching. PathType can be one of the following values: * Exact: Matches the URL path exactly. * Prefix: Matches based on a URL path prefix split by '/'. Matching is
    ///   done on a path element by element basis. A path element refers is the
    ///   list of labels in the path split by the '/' separator. A request is a
    ///   match for path p if every p is an element-wise prefix of p of the
    ///   request path. Note that if the last element of the path is a substring
    ///   of the last element in request path, it is not a match (e.g. /foo/bar
    ///   matches /foo/bar/baz, but does not match /foo/barbaz).
    /// * ImplementationSpecific: Interpretation of the Path matching is up to
    ///   the IngressClass. Implementations can treat this as a separate PathType
    ///   or treat it identically to Prefix or Exact path types.
    /// Implementations are required to support all path types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_type: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::networking::v1::HTTPIngressPath {
    type Optioned = HTTPIngressPathAc;
}
#[automatically_derived]
impl crate::Optionable for HTTPIngressPathAc {
    type Optioned = HTTPIngressPathAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::networking::v1::HTTPIngressPath {
    fn into_optioned(self) -> HTTPIngressPathAc {
        HTTPIngressPathAc {
            backend: Some(crate::OptionableConvert::into_optioned(self.backend)),
            path: self.path,
            path_type: Some(self.path_type),
        }
    }
    fn try_from_optioned(value: HTTPIngressPathAc) -> Result<Self, crate::Error> {
        Ok(Self {
            backend: crate::OptionableConvert::try_from_optioned(
                value
                    .backend
                    .ok_or(crate::Error {
                        missing_field: "backend",
                    })?,
            )?,
            path: value.path,
            path_type: value
                .path_type
                .ok_or(crate::Error {
                    missing_field: "path_type",
                })?,
        })
    }
    fn merge(&mut self, other: HTTPIngressPathAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.backend {
            crate::OptionableConvert::merge(&mut self.backend, other_value)?;
        }
        if other.path.is_some() {
            self.path = other.path;
        }
        if let Some(other_value) = other.path_type {
            self.path_type = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::networking::v1::HTTPIngressPath>
for HTTPIngressPathAc {
    fn from_optionable(
        value: k8s_openapi027::api::networking::v1::HTTPIngressPath,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::networking::v1::HTTPIngressPath, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::networking::v1::HTTPIngressPath,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
