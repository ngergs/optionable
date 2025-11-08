#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct PodCertificateRequestSpecAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_expiration_seconds: <Option<i32> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_uid: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkix_public_key: Option<
        <::k8s_openapi::ByteString as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_uid: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_possession: Option<
        <::k8s_openapi::ByteString as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<
        <std::string::String as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_uid: Option<
        <std::string::String as crate::Optionable>::Optioned,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::certificates::v1alpha1::PodCertificateRequestSpec {
    type Optioned = PodCertificateRequestSpecAc;
}
#[automatically_derived]
impl crate::Optionable for PodCertificateRequestSpecAc {
    type Optioned = PodCertificateRequestSpecAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::certificates::v1alpha1::PodCertificateRequestSpec {
    fn into_optioned(self) -> PodCertificateRequestSpecAc {
        PodCertificateRequestSpecAc {
            max_expiration_seconds: crate::OptionableConvert::into_optioned(
                self.max_expiration_seconds,
            ),
            node_name: Some(crate::OptionableConvert::into_optioned(self.node_name)),
            node_uid: Some(crate::OptionableConvert::into_optioned(self.node_uid)),
            pkix_public_key: Some(
                crate::OptionableConvert::into_optioned(self.pkix_public_key),
            ),
            pod_name: Some(crate::OptionableConvert::into_optioned(self.pod_name)),
            pod_uid: Some(crate::OptionableConvert::into_optioned(self.pod_uid)),
            proof_of_possession: Some(
                crate::OptionableConvert::into_optioned(self.proof_of_possession),
            ),
            service_account_name: Some(
                crate::OptionableConvert::into_optioned(self.service_account_name),
            ),
            service_account_uid: Some(
                crate::OptionableConvert::into_optioned(self.service_account_uid),
            ),
            signer_name: Some(crate::OptionableConvert::into_optioned(self.signer_name)),
        }
    }
    fn try_from_optioned(
        value: PodCertificateRequestSpecAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            max_expiration_seconds: crate::OptionableConvert::try_from_optioned(
                value.max_expiration_seconds,
            )?,
            node_name: crate::OptionableConvert::try_from_optioned(
                value
                    .node_name
                    .ok_or(crate::Error {
                        missing_field: "node_name",
                    })?,
            )?,
            node_uid: crate::OptionableConvert::try_from_optioned(
                value
                    .node_uid
                    .ok_or(crate::Error {
                        missing_field: "node_uid",
                    })?,
            )?,
            pkix_public_key: crate::OptionableConvert::try_from_optioned(
                value
                    .pkix_public_key
                    .ok_or(crate::Error {
                        missing_field: "pkix_public_key",
                    })?,
            )?,
            pod_name: crate::OptionableConvert::try_from_optioned(
                value
                    .pod_name
                    .ok_or(crate::Error {
                        missing_field: "pod_name",
                    })?,
            )?,
            pod_uid: crate::OptionableConvert::try_from_optioned(
                value
                    .pod_uid
                    .ok_or(crate::Error {
                        missing_field: "pod_uid",
                    })?,
            )?,
            proof_of_possession: crate::OptionableConvert::try_from_optioned(
                value
                    .proof_of_possession
                    .ok_or(crate::Error {
                        missing_field: "proof_of_possession",
                    })?,
            )?,
            service_account_name: crate::OptionableConvert::try_from_optioned(
                value
                    .service_account_name
                    .ok_or(crate::Error {
                        missing_field: "service_account_name",
                    })?,
            )?,
            service_account_uid: crate::OptionableConvert::try_from_optioned(
                value
                    .service_account_uid
                    .ok_or(crate::Error {
                        missing_field: "service_account_uid",
                    })?,
            )?,
            signer_name: crate::OptionableConvert::try_from_optioned(
                value
                    .signer_name
                    .ok_or(crate::Error {
                        missing_field: "signer_name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: PodCertificateRequestSpecAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.max_expiration_seconds,
            other.max_expiration_seconds,
        )?;
        if let Some(other_value) = other.node_name {
            crate::OptionableConvert::merge(&mut self.node_name, other_value)?;
        }
        if let Some(other_value) = other.node_uid {
            crate::OptionableConvert::merge(&mut self.node_uid, other_value)?;
        }
        if let Some(other_value) = other.pkix_public_key {
            crate::OptionableConvert::merge(&mut self.pkix_public_key, other_value)?;
        }
        if let Some(other_value) = other.pod_name {
            crate::OptionableConvert::merge(&mut self.pod_name, other_value)?;
        }
        if let Some(other_value) = other.pod_uid {
            crate::OptionableConvert::merge(&mut self.pod_uid, other_value)?;
        }
        if let Some(other_value) = other.proof_of_possession {
            crate::OptionableConvert::merge(&mut self.proof_of_possession, other_value)?;
        }
        if let Some(other_value) = other.service_account_name {
            crate::OptionableConvert::merge(
                &mut self.service_account_name,
                other_value,
            )?;
        }
        if let Some(other_value) = other.service_account_uid {
            crate::OptionableConvert::merge(&mut self.service_account_uid, other_value)?;
        }
        if let Some(other_value) = other.signer_name {
            crate::OptionableConvert::merge(&mut self.signer_name, other_value)?;
        }
        Ok(())
    }
}
