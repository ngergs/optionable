pub struct PreconditionsAc {
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Preconditions {
    type Optioned = PreconditionsAc;
}
#[automatically_derived]
impl crate::Optionable for PreconditionsAc {
    type Optioned = PreconditionsAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::Preconditions {
    fn into_optioned(self) -> PreconditionsAc {
        PreconditionsAc {
            resource_version: crate::OptionableConvert::into_optioned(
                self.resource_version,
            ),
            uid: crate::OptionableConvert::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(
        value: PreconditionsAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            resource_version: crate::OptionableConvert::try_from_optioned(
                value.resource_version,
            )?,
            uid: crate::OptionableConvert::try_from_optioned(value.uid)?,
        })
    }
    fn merge(&mut self, other: PreconditionsAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        crate::OptionableConvert::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
