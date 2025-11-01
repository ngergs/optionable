pub struct RollingUpdateDeploymentAc {
    pub max_surge: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
    pub max_unavailable: <Option<
        ::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::apps::v1::RollingUpdateDeployment {
    type Optioned = RollingUpdateDeploymentAc;
}
#[automatically_derived]
impl crate::Optionable for RollingUpdateDeploymentAc {
    type Optioned = RollingUpdateDeploymentAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::apps::v1::RollingUpdateDeployment {
    fn into_optioned(self) -> RollingUpdateDeploymentAc {
        RollingUpdateDeploymentAc {
            max_surge: crate::OptionableConvert::into_optioned(self.max_surge),
            max_unavailable: crate::OptionableConvert::into_optioned(
                self.max_unavailable,
            ),
        }
    }
    fn try_from_optioned(
        value: RollingUpdateDeploymentAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            max_surge: crate::OptionableConvert::try_from_optioned(value.max_surge)?,
            max_unavailable: crate::OptionableConvert::try_from_optioned(
                value.max_unavailable,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: RollingUpdateDeploymentAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.max_surge, other.max_surge)?;
        crate::OptionableConvert::merge(
            &mut self.max_unavailable,
            other.max_unavailable,
        )?;
        Ok(())
    }
}
