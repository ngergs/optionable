#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LifecycleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_start: <Option<
        ::k8s_openapi026::api::core::v1::LifecycleHandler,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_stop: <Option<
        ::k8s_openapi026::api::core::v1::LifecycleHandler,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi026::api::core::v1::Lifecycle {
    type Optioned = LifecycleAc;
}
#[automatically_derived]
impl crate::Optionable for LifecycleAc {
    type Optioned = LifecycleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi026::api::core::v1::Lifecycle {
    fn into_optioned(self) -> LifecycleAc {
        LifecycleAc {
            post_start: crate::OptionableConvert::into_optioned(self.post_start),
            pre_stop: crate::OptionableConvert::into_optioned(self.pre_stop),
        }
    }
    fn try_from_optioned(value: LifecycleAc) -> Result<Self, crate::Error> {
        Ok(Self {
            post_start: crate::OptionableConvert::try_from_optioned(value.post_start)?,
            pre_stop: crate::OptionableConvert::try_from_optioned(value.pre_stop)?,
        })
    }
    fn merge(&mut self, other: LifecycleAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.post_start, other.post_start)?;
        crate::OptionableConvert::merge(&mut self.pre_stop, other.pre_stop)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi026::api::core::v1::Lifecycle> for LifecycleAc {
    fn from_optionable(value: k8s_openapi026::api::core::v1::Lifecycle) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi026::api::core::v1::Lifecycle, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi026::api::core::v1::Lifecycle,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
