#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// ImageVolumeSource represents a image volume resource.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ImageVolumeSourceAc {
    /// Policy for pulling OCI objects. Possible values are: Always: the kubelet always attempts to pull the reference. Container creation will fail If the pull fails. Never: the kubelet never pulls the reference and only uses a local image or artifact. Container creation will fail if the reference isn't present. IfNotPresent: the kubelet pulls if the reference isn't already present on disk. Container creation will fail if the reference isn't present and the pull fails. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_policy: Option<std::string::String>,
    /// Required: Image or artifact reference to be used. Behaves in the same way as pod.spec.containers\[*\].image. Pull secrets will be assembled in the same way as for the container image by looking up node credentials, SA image pull secrets, and pod spec image pull secrets. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::ImageVolumeSource {
    type Optioned = ImageVolumeSourceAc;
}
#[automatically_derived]
impl crate::Optionable for ImageVolumeSourceAc {
    type Optioned = ImageVolumeSourceAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::ImageVolumeSource {
    fn into_optioned(self) -> ImageVolumeSourceAc {
        ImageVolumeSourceAc {
            pull_policy: self.pull_policy,
            reference: self.reference,
        }
    }
    fn try_from_optioned(value: ImageVolumeSourceAc) -> Result<Self, crate::Error> {
        Ok(Self {
            pull_policy: value.pull_policy,
            reference: value.reference,
        })
    }
    fn merge(&mut self, other: ImageVolumeSourceAc) -> Result<(), crate::Error> {
        if other.pull_policy.is_some() {
            self.pull_policy = other.pull_policy;
        }
        if other.reference.is_some() {
            self.reference = other.reference;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::ImageVolumeSource>
for ImageVolumeSourceAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::ImageVolumeSource) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::ImageVolumeSource, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::ImageVolumeSource,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
