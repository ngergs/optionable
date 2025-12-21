#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceTaintRuleAc {
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
    pub spec: Option<
        <::k8s_openapi026::api::resource::v1alpha3::DeviceTaintRuleSpec as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule {
    type Optioned = DeviceTaintRuleAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintRuleAc {
    type Optioned = DeviceTaintRuleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule {
    fn into_optioned(self) -> DeviceTaintRuleAc {
        DeviceTaintRuleAc {
            api_version: Default::default(),
            kind: Default::default(),
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(value: DeviceTaintRuleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: DeviceTaintRuleAc) -> Result<(), crate::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule>
for DeviceTaintRuleAc {
    fn from_optionable(
        value: k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi026::Resource for DeviceTaintRuleAc {
    const API_VERSION: &'static str = <k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule as k8s_openapi026::Resource>::API_VERSION;
    const GROUP: &'static str = <k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule as k8s_openapi026::Resource>::GROUP;
    const KIND: &'static str = <k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule as k8s_openapi026::Resource>::KIND;
    const VERSION: &'static str = <k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule as k8s_openapi026::Resource>::VERSION;
    const URL_PATH_SEGMENT: &'static str = <k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule as k8s_openapi026::Resource>::URL_PATH_SEGMENT;
    type Scope = <k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule as k8s_openapi026::Resource>::Scope;
}
impl k8s_openapi026::Metadata for DeviceTaintRuleAc {
    type Ty = <k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule as k8s_openapi026::Metadata>::Ty;
    fn metadata(&self) -> &<Self as k8s_openapi026::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi026::Metadata>::Ty {
        &mut self.metadata
    }
}
#[cfg(test_k8s_openapi_roundtrip)]
#[test]
fn roundtrip_devicetaintruleac() {
    crate::testutil::roundtrip_test::<
        k8s_openapi026::api::resource::v1alpha3::DeviceTaintRule,
    >();
}
