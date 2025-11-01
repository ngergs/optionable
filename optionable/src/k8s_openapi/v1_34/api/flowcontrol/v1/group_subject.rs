pub struct GroupSubjectAc {
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::flowcontrol::v1::GroupSubject {
    type Optioned = GroupSubjectAc;
}
#[automatically_derived]
impl crate::Optionable for GroupSubjectAc {
    type Optioned = GroupSubjectAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::flowcontrol::v1::GroupSubject {
    fn into_optioned(self) -> GroupSubjectAc {
        GroupSubjectAc {
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
        }
    }
    fn try_from_optioned(
        value: GroupSubjectAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: GroupSubjectAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        Ok(())
    }
}
