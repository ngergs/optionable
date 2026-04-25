#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// CertificateSigningRequestCondition describes a condition of a CertificateSigningRequest object
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CertificateSigningRequestConditionAc {
    /// lastTransitionTime is the time the condition last transitioned from one status to another. If unset, when a new condition type is added or an existing condition's status is changed, the server defaults this to the current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// lastUpdateTime is the time of the last update to this condition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// message contains a human readable message with details about the request state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<std::string::String>,
    /// reason indicates a brief reason for the request state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<std::string::String>,
    /// status of the condition, one of True, False, Unknown. Approved, Denied, and Failed conditions may not be "False" or "Unknown".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::string::String>,
    /// type of the condition. Known conditions are "Approved", "Denied", and "Failed".
    ///
    /// An "Approved" condition is added via the /approval subresource, indicating the request was approved and should be issued by the signer.
    ///
    /// A "Denied" condition is added via the /approval subresource, indicating the request was denied and should not be issued by the signer.
    ///
    /// A "Failed" condition is added via the /status subresource, indicating the signer failed to issue the certificate.
    ///
    /// Approved and Denied conditions are mutually exclusive. Approved, Denied, and Failed conditions cannot be removed once added.
    ///
    /// Only one condition of a given type is allowed.
    #[serde(rename = "type")]
    pub type_: std::string::String,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::api::certificates::v1::CertificateSigningRequestCondition {
    type Optioned = CertificateSigningRequestConditionAc;
}
#[automatically_derived]
impl crate::Optionable for CertificateSigningRequestConditionAc {
    type Optioned = CertificateSigningRequestConditionAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::api::certificates::v1::CertificateSigningRequestCondition {
    fn into_optioned(self) -> CertificateSigningRequestConditionAc {
        CertificateSigningRequestConditionAc {
            last_transition_time: crate::OptionableConvert::into_optioned(
                self.last_transition_time,
            ),
            last_update_time: crate::OptionableConvert::into_optioned(
                self.last_update_time,
            ),
            message: self.message,
            reason: self.reason,
            status: Some(self.status),
            type_: self.type_,
        }
    }
    fn try_from_optioned(
        value: CertificateSigningRequestConditionAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            last_transition_time: crate::OptionableConvert::try_from_optioned(
                value.last_transition_time,
            )?,
            last_update_time: crate::OptionableConvert::try_from_optioned(
                value.last_update_time,
            )?,
            message: value.message,
            reason: value.reason,
            status: value
                .status
                .ok_or(crate::Error {
                    missing_field: "status",
                })?,
            type_: value.type_,
        })
    }
    fn merge(
        &mut self,
        other: CertificateSigningRequestConditionAc,
    ) -> Result<(), crate::Error> {
        if self.last_transition_time.is_none() {
            self.last_transition_time = crate::OptionableConvert::try_from_optioned(
                other.last_transition_time,
            )?;
        } else if let Some(self_value) = self.last_transition_time.as_mut()
            && let Some(other_value) = other.last_transition_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.last_update_time.is_none() {
            self.last_update_time = crate::OptionableConvert::try_from_optioned(
                other.last_update_time,
            )?;
        } else if let Some(self_value) = self.last_update_time.as_mut()
            && let Some(other_value) = other.last_update_time
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.message.is_none() {
            self.message = crate::OptionableConvert::try_from_optioned(other.message)?;
        } else if let Some(self_value) = self.message.as_mut()
            && let Some(other_value) = other.message
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.reason.is_none() {
            self.reason = crate::OptionableConvert::try_from_optioned(other.reason)?;
        } else if let Some(self_value) = self.reason.as_mut()
            && let Some(other_value) = other.reason
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if let Some(other_value) = other.status {
            self.status = crate::OptionableConvert::try_from_optioned(other_value)?;
        }
        self.type_ = other.type_;
        Ok(())
    }
}
#[automatically_derived]
impl crate::merge::OptionableMapKeysEq
for k8s_openapi027::api::certificates::v1::CertificateSigningRequestCondition {
    fn keys_eq(&self, other: &<Self as crate::Optionable>::Optioned) -> bool {
        self.type_ == other.type_
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::api::certificates::v1::CertificateSigningRequestCondition,
> for CertificateSigningRequestConditionAc {
    fn from_optionable(
        value: k8s_openapi027::api::certificates::v1::CertificateSigningRequestCondition,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::api::certificates::v1::CertificateSigningRequestCondition,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::certificates::v1::CertificateSigningRequestCondition,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi027::DeepMerge for CertificateSigningRequestConditionAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.last_transition_time,
            other.last_transition_time,
        );
        k8s_openapi027::DeepMerge::merge_from(
            &mut self.last_update_time,
            other.last_update_time,
        );
        k8s_openapi027::DeepMerge::merge_from(&mut self.message, other.message);
        k8s_openapi027::DeepMerge::merge_from(&mut self.reason, other.reason);
        k8s_openapi027::DeepMerge::merge_from(&mut self.status, other.status);
        k8s_openapi027::DeepMerge::merge_from(&mut self.type_, other.type_);
    }
}
impl crate::merge::MapKeysEq for CertificateSigningRequestConditionAc {
    fn keys_eq(&self, other: &Self) -> bool {
        self.type_ == other.type_
    }
}
