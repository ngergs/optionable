pub struct StatefulSetOrdinalsOpt {
    pub start: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::StatefulSetOrdinals {
    type Optioned = StatefulSetOrdinalsOpt;
}
#[automatically_derived]
impl crate::Optionable for StatefulSetOrdinalsOpt {
    type Optioned = StatefulSetOrdinalsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::StatefulSetOrdinals {
    fn into_optioned(self) -> StatefulSetOrdinalsOpt {
        StatefulSetOrdinalsOpt {
            start: <Option<i32> as crate::OptionableConvert>::into_optioned(self.start),
        }
    }
    fn try_from_optioned(
        value: StatefulSetOrdinalsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            start: <Option<
                i32,
            > as crate::OptionableConvert>::try_from_optioned(value.start)?,
        })
    }
    fn merge(
        &mut self,
        other: StatefulSetOrdinalsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<i32> as crate::OptionableConvert>::merge(&mut self.start, other.start)?;
        Ok(())
    }
}
