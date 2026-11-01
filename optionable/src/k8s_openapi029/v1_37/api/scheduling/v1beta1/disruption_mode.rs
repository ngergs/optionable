#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
/// DisruptionMode defines how individual entities within a group can be disrupted. Exactly one mode can be set.
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DisruptionModeAc {
    /// all specifies that all children can only be disrupted together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<
        <::k8s_openapi029::api::scheduling::v1beta1::AllDisruptionMode as crate::Optionable>::Optioned,
    >,
    /// single specifies that children can be disrupted independently from each other.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single: Option<
        <::k8s_openapi029::api::scheduling::v1beta1::SingleDisruptionMode as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi029::api::scheduling::v1beta1::DisruptionMode {
    type Optioned = DisruptionModeAc;
}
#[automatically_derived]
impl crate::Optionable for DisruptionModeAc {
    type Optioned = DisruptionModeAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi029::api::scheduling::v1beta1::DisruptionMode {
    fn into_optioned(self) -> DisruptionModeAc {
        DisruptionModeAc {
            all: crate::OptionableConvert::into_optioned(self.all),
            single: crate::OptionableConvert::into_optioned(self.single),
        }
    }
    fn try_from_optioned(value: DisruptionModeAc) -> Result<Self, crate::Error> {
        Ok(Self {
            all: crate::OptionableConvert::try_from_optioned(value.all)?,
            single: crate::OptionableConvert::try_from_optioned(value.single)?,
        })
    }
    fn merge(&mut self, other: DisruptionModeAc) -> Result<(), crate::Error> {
        if self.all.is_none() {
            self.all = crate::OptionableConvert::try_from_optioned(other.all)?;
        } else if let Some(self_value) = self.all.as_mut()
            && let Some(other_value) = other.all
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        if self.single.is_none() {
            self.single = crate::OptionableConvert::try_from_optioned(other.single)?;
        } else if let Some(self_value) = self.single.as_mut()
            && let Some(other_value) = other.single
        {
            crate::OptionableConvert::merge(self_value, other_value)?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi029::api::scheduling::v1beta1::DisruptionMode>
for DisruptionModeAc {
    fn from_optionable(
        value: k8s_openapi029::api::scheduling::v1beta1::DisruptionMode,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi029::api::scheduling::v1beta1::DisruptionMode, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi029::api::scheduling::v1beta1::DisruptionMode,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
impl k8s_openapi029::DeepMerge for DisruptionModeAc {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi029::DeepMerge::merge_from(&mut self.all, other.all);
        k8s_openapi029::DeepMerge::merge_from(&mut self.single, other.single);
    }
}
