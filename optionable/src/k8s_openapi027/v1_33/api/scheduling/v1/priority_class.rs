#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// PriorityClass defines mapping from a priority class name to the priority integer value. The value can be any valid integer.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PriorityClassAc {
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_api_version",
        deserialize_with = "crate::k8s_openapi::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi::serialize_kind",
        deserialize_with = "crate::k8s_openapi::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    /// description is an arbitrary string that usually provides guidelines on when this priority class should be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<std::string::String>,
    /// globalDefault specifies whether this PriorityClass should be considered as the default priority for pods that do not have any priority class. Only one PriorityClass can be marked as `globalDefault`. However, if more than one PriorityClasses exists with their `globalDefault` field set to true, the smallest value of such global default PriorityClasses will be used as the default priority.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_default: Option<bool>,
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    pub metadata: ::k8s_openapi027::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    /// preemptionPolicy is the Policy for preempting pods with lower priority. One of Never, PreemptLowerPriority. Defaults to PreemptLowerPriority if unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preemption_policy: Option<std::string::String>,
    /// value represents the integer value of this priority class. This is the actual priority that pods receive when they have the name of this class in their pod spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::scheduling::v1::PriorityClass {
    type Optioned = PriorityClassAc;
}
#[automatically_derived]
impl crate::Optionable for PriorityClassAc {
    type Optioned = PriorityClassAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::scheduling::v1::PriorityClass {
    fn into_optioned(self) -> PriorityClassAc {
        PriorityClassAc {
            api_version: Default::default(),
            kind: Default::default(),
            description: self.description,
            global_default: self.global_default,
            metadata: self.metadata,
            preemption_policy: self.preemption_policy,
            value: Some(self.value),
        }
    }
    fn try_from_optioned(value: PriorityClassAc) -> Result<Self, crate::Error> {
        Ok(Self {
            description: value.description,
            global_default: value.global_default,
            metadata: value.metadata,
            preemption_policy: value.preemption_policy,
            value: value
                .value
                .ok_or(crate::Error {
                    missing_field: "value",
                })?,
        })
    }
    fn merge(&mut self, other: PriorityClassAc) -> Result<(), crate::Error> {
        self.description = other.description;
        self.global_default = other.global_default;
        self.metadata = other.metadata;
        self.preemption_policy = other.preemption_policy;
        if let Some(other_value) = other.value {
            self.value = other_value;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::scheduling::v1::PriorityClass>
for PriorityClassAc {
    fn from_optionable(
        value: k8s_openapi027::api::scheduling::v1::PriorityClass,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::scheduling::v1::PriorityClass, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::scheduling::v1::PriorityClass,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::Resource for PriorityClassAc {
    const API_VERSION: &'static str = <k8s_openapi027::api::scheduling::v1::PriorityClass as k8s_openapi027::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi027::api::scheduling::v1::PriorityClass as k8s_openapi027::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi027::api::scheduling::v1::PriorityClass as k8s_openapi027::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi027::api::scheduling::v1::PriorityClass as k8s_openapi027::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi027::api::scheduling::v1::PriorityClass as k8s_openapi027::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi027::api::scheduling::v1::PriorityClass as k8s_openapi027::Resource>::Scope;
}
impl k8s_openapi027::Metadata for PriorityClassAc {
    type Ty = <k8s_openapi027::api::scheduling::v1::PriorityClass as k8s_openapi027::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi027::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi027::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_priorityclassac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi027::api::scheduling::v1::PriorityClass,
    >();
}
