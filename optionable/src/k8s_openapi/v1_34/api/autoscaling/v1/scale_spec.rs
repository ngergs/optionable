pub struct ScaleSpecOpt {
    pub replicas: <Option<i32> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::autoscaling::v1::ScaleSpec {
    type Optioned = ScaleSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for ScaleSpecOpt {
    type Optioned = ScaleSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::autoscaling::v1::ScaleSpec {
    fn into_optioned(self) -> ScaleSpecOpt {
        ScaleSpecOpt {
            replicas: crate::OptionableConvert::into_optioned(self.replicas),
        }
    }
    fn try_from_optioned(value: ScaleSpecOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            replicas: crate::OptionableConvert::try_from_optioned(value.replicas)?,
        })
    }
    fn merge(&mut self, other: ScaleSpecOpt) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.replicas, other.replicas)?;
        Ok(())
    }
}
