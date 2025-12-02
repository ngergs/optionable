#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceQuotaStatusAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: <Option<
        std::collections::BTreeMap<
            std::string::String,
            ::k8s_openapi::apimachinery::pkg::api::resource::Quantity,
        >,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::core::v1::ResourceQuotaStatus {
    type Optioned = ResourceQuotaStatusAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceQuotaStatusAc {
    type Optioned = ResourceQuotaStatusAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for ::k8s_openapi::api::core::v1::ResourceQuotaStatus {
    fn into_optioned(self) -> ResourceQuotaStatusAc {
        ResourceQuotaStatusAc {
            hard: crate::OptionableConvert::into_optioned(self.hard),
            used: crate::OptionableConvert::into_optioned(self.used),
        }
    }
    fn try_from_optioned(value: ResourceQuotaStatusAc) -> Result<Self, crate::Error> {
        Ok(Self {
            hard: crate::OptionableConvert::try_from_optioned(value.hard)?,
            used: crate::OptionableConvert::try_from_optioned(value.used)?,
        })
    }
    fn merge(&mut self, other: ResourceQuotaStatusAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.hard, other.hard)?;
        crate::OptionableConvert::merge(&mut self.used, other.used)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<::k8s_openapi::api::core::v1::ResourceQuotaStatus>
for ResourceQuotaStatusAc {
    fn from_optionable(
        value: ::k8s_openapi::api::core::v1::ResourceQuotaStatus,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<::k8s_openapi::api::core::v1::ResourceQuotaStatus, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut ::k8s_openapi::api::core::v1::ResourceQuotaStatus,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
