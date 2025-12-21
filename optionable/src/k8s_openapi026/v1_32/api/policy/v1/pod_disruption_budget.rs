#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PodDisruptionBudgetAc {
    #[serde(
        serialize_with = "crate::k8s_openapi026::serialize_api_version",
        deserialize_with = "crate::k8s_openapi026::deserialize_api_version"
    )]
    pub api_version: std::marker::PhantomData<Self>,
    #[serde(
        serialize_with = "crate::k8s_openapi026::serialize_kind",
        deserialize_with = "crate::k8s_openapi026::deserialize_kind"
    )]
    pub kind: std::marker::PhantomData<Self>,
    pub metadata: ::k8s_openapi026::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: <Option<
        ::k8s_openapi026::api::policy::v1::PodDisruptionBudgetSpec,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: <Option<
        ::k8s_openapi026::api::policy::v1::PodDisruptionBudgetStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::policy::v1::PodDisruptionBudget {
    type Optioned = PodDisruptionBudgetAc;
}
#[automatically_derived]
impl crate::Optionable for PodDisruptionBudgetAc {
    type Optioned = PodDisruptionBudgetAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::policy::v1::PodDisruptionBudget {
    fn into_optioned(self) -> PodDisruptionBudgetAc {
        PodDisruptionBudgetAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: crate::OptionableConvert::into_optioned(self.spec),
            status: crate::OptionableConvert::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: PodDisruptionBudgetAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(value.spec)?,
            status: crate::OptionableConvert::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: PodDisruptionBudgetAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        crate::OptionableConvert::merge(&mut self.spec, other.spec)?;
        crate::OptionableConvert::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::policy::v1::PodDisruptionBudget>
for PodDisruptionBudgetAc {
    fn from_optionable(
        value: k8s_openapi026::api::policy::v1::PodDisruptionBudget,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::policy::v1::PodDisruptionBudget, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::policy::v1::PodDisruptionBudget,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi026::Resource for PodDisruptionBudgetAc {
    const API_VERSION: &'static str = <k8s_openapi026::api::policy::v1::PodDisruptionBudget as k8s_openapi026::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi026::api::policy::v1::PodDisruptionBudget as k8s_openapi026::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi026::api::policy::v1::PodDisruptionBudget as k8s_openapi026::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi026::api::policy::v1::PodDisruptionBudget as k8s_openapi026::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi026::api::policy::v1::PodDisruptionBudget as k8s_openapi026::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi026::api::policy::v1::PodDisruptionBudget as k8s_openapi026::Resource>::Scope;
}
impl k8s_openapi026::Metadata for PodDisruptionBudgetAc {
    type Ty = <k8s_openapi026::api::policy::v1::PodDisruptionBudget as k8s_openapi026::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi026::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi026::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_poddisruptionbudgetac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi026::api::policy::v1::PodDisruptionBudget,
    >();
}
