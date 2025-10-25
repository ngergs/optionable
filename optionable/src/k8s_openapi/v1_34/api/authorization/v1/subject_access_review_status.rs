pub struct SubjectAccessReviewStatusOpt {
    pub allowed: Option<bool>,
    pub denied: <Option<bool> as crate::Optionable>::Optioned,
    pub evaluation_error: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub reason: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::authorization::v1::SubjectAccessReviewStatus {
    type Optioned = SubjectAccessReviewStatusOpt;
}
#[automatically_derived]
impl crate::Optionable for SubjectAccessReviewStatusOpt {
    type Optioned = SubjectAccessReviewStatusOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::SubjectAccessReviewStatus {
    fn into_optioned(self) -> SubjectAccessReviewStatusOpt {
        SubjectAccessReviewStatusOpt {
            allowed: Some(self.allowed),
            denied: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.denied),
            evaluation_error: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.evaluation_error),
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.reason),
        }
    }
    fn try_from_optioned(
        value: SubjectAccessReviewStatusOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            allowed: value
                .allowed
                .ok_or(crate::optionable::Error {
                    missing_field: "allowed",
                })?,
            denied: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(value.denied)?,
            evaluation_error: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.evaluation_error)?,
            reason: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.reason)?,
        })
    }
    fn merge(
        &mut self,
        other: SubjectAccessReviewStatusOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.allowed {
            self.allowed = other_value;
        }
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(&mut self.denied, other.denied)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.evaluation_error,
            other.evaluation_error,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.reason, other.reason)?;
        Ok(())
    }
}
