#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSIDriverAc {
    pub metadata: ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<
        <::k8s_openapi::api::storage::v1::CSIDriverSpec as crate::Optionable>::Optioned,
    >,
    #[serde(
        flatten,
        serialize_with = "crate::k8s_openapi::serialize_api_envelope",
        skip_deserializing
    )]
    phantom: std::marker::PhantomData<CSIDriverAc>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::storage::v1::CSIDriver {
    type Optioned = CSIDriverAc;
}
#[automatically_derived]
impl crate::Optionable for CSIDriverAc {
    type Optioned = CSIDriverAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::storage::v1::CSIDriver {
    fn into_optioned(self) -> CSIDriverAc {
        CSIDriverAc {
            metadata: self.metadata,
            spec: Some(crate::OptionableConvert::into_optioned(self.spec)),
        }
    }
    fn try_from_optioned(value: CSIDriverAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: value.metadata,
            spec: crate::OptionableConvert::try_from_optioned(
                value
                    .spec
                    .ok_or(crate::optionable::Error {
                        missing_field: "spec",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: CSIDriverAc) -> Result<(), crate::optionable::Error> {
        self.metadata = other.metadata;
        if let Some(other_value) = other.spec {
            crate::OptionableConvert::merge(&mut self.spec, other_value)?;
        }
        Ok(())
    }
}
impl k8s_openapi::Resource for CSIDriverAc {
    const API_VERSION: &'static str = "storage.k8s.io/v1";
    const GROUP: &'static str = "storage.k8s.io";
    const KIND: &'static str = "CSIDriver";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "csidrivers";
    type Scope = k8s_openapi::ClusterResourceScope;
}
impl k8s_openapi::Metadata for CSIDriverAc {
    type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }
    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}
