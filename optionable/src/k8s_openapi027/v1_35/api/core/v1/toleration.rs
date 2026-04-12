#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// The pod this Toleration is attached to tolerates any taint that matches the triple \<key,value,effect\> using the matching operator \<operator\>.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TolerationAc {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<std::string::String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists, Equal, Lt, and Gt. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category. Lt and Gt perform numeric comparisons (requires feature gate TaintTolerationComparisonOperators).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<std::string::String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::core::v1::Toleration {
    type Optioned = TolerationAc;
}
#[automatically_derived]
impl crate::Optionable for TolerationAc {
    type Optioned = TolerationAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::core::v1::Toleration {
    fn into_optioned(self) -> TolerationAc {
        TolerationAc {
            effect: self.effect,
            key: self.key,
            operator: self.operator,
            toleration_seconds: self.toleration_seconds,
            value: self.value,
        }
    }
    fn try_from_optioned(value: TolerationAc) -> Result<Self, crate::Error> {
        Ok(Self {
            effect: value.effect,
            key: value.key,
            operator: value.operator,
            toleration_seconds: value.toleration_seconds,
            value: value.value,
        })
    }
    fn merge(&mut self, other: TolerationAc) -> Result<(), crate::Error> {
        if self.effect.is_none() {
            self.effect = crate::OptionableConvert::try_from_optioned(other.effect)?;
        } else {
            crate::OptionableConvert::merge(&mut self.effect, other.effect)?;
        }
        if self.key.is_none() {
            self.key = crate::OptionableConvert::try_from_optioned(other.key)?;
        } else {
            crate::OptionableConvert::merge(&mut self.key, other.key)?;
        }
        if self.operator.is_none() {
            self.operator = crate::OptionableConvert::try_from_optioned(other.operator)?;
        } else {
            crate::OptionableConvert::merge(&mut self.operator, other.operator)?;
        }
        if self.toleration_seconds.is_none() {
            self.toleration_seconds = crate::OptionableConvert::try_from_optioned(
                other.toleration_seconds,
            )?;
        } else {
            crate::OptionableConvert::merge(
                &mut self.toleration_seconds,
                other.toleration_seconds,
            )?;
        }
        if self.value.is_none() {
            self.value = crate::OptionableConvert::try_from_optioned(other.value)?;
        } else {
            crate::OptionableConvert::merge(&mut self.value, other.value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::core::v1::Toleration> for TolerationAc {
    fn from_optionable(value: k8s_openapi027::api::core::v1::Toleration) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::core::v1::Toleration, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::core::v1::Toleration,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
