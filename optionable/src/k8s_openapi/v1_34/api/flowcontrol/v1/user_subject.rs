pub struct UserSubjectOpt {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::flowcontrol::v1::user_subject::UserSubject {
    type Optioned = UserSubjectOpt;
}
#[automatically_derived]
impl crate::Optionable for UserSubjectOpt {
    type Optioned = UserSubjectOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::flowcontrol::v1::user_subject::UserSubject {
    fn into_optioned(self) -> UserSubjectOpt {
        UserSubjectOpt {
            name: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.name,
                ),
            ),
        }
    }
    fn try_from_optioned(
        value: UserSubjectOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: UserSubjectOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.name,
                other_value,
            )?;
        }
        Ok(())
    }
}
