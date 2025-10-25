pub struct PreconditionsOpt {
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub uid: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::preconditions::Preconditions {
    type Optioned = PreconditionsOpt;
}
#[automatically_derived]
impl crate::Optionable for PreconditionsOpt {
    type Optioned = PreconditionsOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::preconditions::Preconditions {
    fn into_optioned(self) -> PreconditionsOpt {
        PreconditionsOpt {
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.resource_version),
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.uid),
        }
    }
    fn try_from_optioned(
        value: PreconditionsOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_version)?,
            uid: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.uid)?,
        })
    }
    fn merge(
        &mut self,
        other: PreconditionsOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.uid, other.uid)?;
        Ok(())
    }
}
