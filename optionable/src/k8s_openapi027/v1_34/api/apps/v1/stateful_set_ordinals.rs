#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StatefulSetOrdinalsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for k8s_openapi027::api::apps::v1::StatefulSetOrdinals {
    type Optioned = StatefulSetOrdinalsAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetOrdinalsAc {
    type Optioned = StatefulSetOrdinalsAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert for k8s_openapi027::api::apps::v1::StatefulSetOrdinals {
    fn into_optioned(self) -> StatefulSetOrdinalsAc {
        StatefulSetOrdinalsAc {
            start: crate::OptionableConvert::into_optioned(self.start),
        }
    }
    fn try_from_optioned(value: StatefulSetOrdinalsAc) -> Result<Self, crate::Error> {
        Ok(Self {
            start: crate::OptionableConvert::try_from_optioned(value.start)?,
        })
    }
    fn merge(&mut self, other: StatefulSetOrdinalsAc) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(&mut self.start, other.start)?;
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<k8s_openapi027::api::apps::v1::StatefulSetOrdinals>
for StatefulSetOrdinalsAc {
    fn from_optionable(
        value: k8s_openapi027::api::apps::v1::StatefulSetOrdinals,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<k8s_openapi027::api::apps::v1::StatefulSetOrdinals, crate::Error> {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::api::apps::v1::StatefulSetOrdinals,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
