pub struct UncountedTerminatedPodsOpt {
    pub failed: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
    pub succeeded: <Option<
        std::vec::Vec<std::string::String>,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::batch::v1::UncountedTerminatedPods {
    type Optioned = UncountedTerminatedPodsOpt;
}
#[automatically_derived]
impl crate::Optionable for UncountedTerminatedPodsOpt {
    type Optioned = UncountedTerminatedPodsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::batch::v1::UncountedTerminatedPods {
    fn into_optioned(self) -> UncountedTerminatedPodsOpt {
        UncountedTerminatedPodsOpt {
            failed: crate::OptionableConvert::into_optioned(self.failed),
            succeeded: crate::OptionableConvert::into_optioned(self.succeeded),
        }
    }
    fn try_from_optioned(
        value: UncountedTerminatedPodsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            failed: crate::OptionableConvert::try_from_optioned(value.failed)?,
            succeeded: crate::OptionableConvert::try_from_optioned(value.succeeded)?,
        })
    }
    fn merge(
        &mut self,
        other: UncountedTerminatedPodsOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.failed, other.failed)?;
        crate::OptionableConvert::merge(&mut self.succeeded, other.succeeded)?;
        Ok(())
    }
}
