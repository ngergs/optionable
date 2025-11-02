#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct StatefulSetOrdinalsAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::StatefulSetOrdinals {
    type Optioned = StatefulSetOrdinalsAc;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetOrdinalsAc {
    type Optioned = StatefulSetOrdinalsAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::StatefulSetOrdinals {
    fn into_optioned(self) -> StatefulSetOrdinalsAc {
        StatefulSetOrdinalsAc {
            start: crate::OptionableConvert::into_optioned(self.start),
        }
    }
    fn try_from_optioned(
        value: StatefulSetOrdinalsAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            start: crate::OptionableConvert::try_from_optioned(value.start)?,
        })
    }
    fn merge(
        &mut self,
        other: StatefulSetOrdinalsAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.start, other.start)?;
        Ok(())
    }
}
