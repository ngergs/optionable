#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// The device this taint is attached to has the "effect" on any claim which does not tolerate the taint and, through the claim, to pods using the claim.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeviceTaintAc {
    /// The effect of the taint on claims that do not tolerate the taint and through such claims on the pods using them.
    ///
    /// Valid effects are None, NoSchedule and NoExecute. PreferNoSchedule as used for nodes is not valid here. More effects may get added in the future. Consumers must treat unknown effects like None.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<std::string::String>,
    /// The taint key to be applied to a device. Must be a label name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<std::string::String>,
    /// TimeAdded represents the time at which the taint was added. Added automatically during create or update if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_added: Option<
        <::k8s_openapi027::apimachinery::pkg::apis::meta::v1::Time as crate::Optionable>::Optioned,
    >,
    /// The taint value corresponding to the taint key. Must be a label value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<std::string::String>,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::resource::v1beta2::DeviceTaint {
    type Optioned = DeviceTaintAc;
}
#[automatically_derived]
impl crate::Optionable for DeviceTaintAc {
    type Optioned = DeviceTaintAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::resource::v1beta2::DeviceTaint {
    fn into_optioned(self) -> DeviceTaintAc {
        DeviceTaintAc {
            effect: Some(self.effect),
            key: Some(self.key),
            time_added: crate::OptionableConvert::into_optioned(self.time_added),
            value: self.value,
        }
    }
    fn try_from_optioned(value: DeviceTaintAc) -> Result<Self, crate::Error> {
        Ok(Self {
            effect: value
                .effect
                .ok_or(crate::Error {
                    missing_field: "effect",
                })?,
            key: value
                .key
                .ok_or(crate::Error {
                    missing_field: "key",
                })?,
            time_added: crate::OptionableConvert::try_from_optioned(value.time_added)?,
            value: value.value,
        })
    }
    fn merge(&mut self, other: DeviceTaintAc) -> Result<(), crate::Error> {
        if let Some(other_value) = other.effect {
            self.effect = other_value;
        }
        if let Some(other_value) = other.key {
            self.key = other_value;
        }
        if self.time_added.is_none() {
            self.time_added = other.time_added;
        }
        if let Some(other_value) = other.time_added {
            crate::OptionableConvert::merge(&mut self.time_added, other_value)?;
        }
        if self.value.is_none() {
            self.value = other.value;
        }
        if let Some(other_value) = other.value {
            crate::OptionableConvert::merge(&mut self.value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::resource::v1beta2::DeviceTaint>
for DeviceTaintAc {
    fn from_optionable(
        value: k8s_openapi027::api::resource::v1beta2::DeviceTaint,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::resource::v1beta2::DeviceTaint, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::resource::v1beta2::DeviceTaint,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
