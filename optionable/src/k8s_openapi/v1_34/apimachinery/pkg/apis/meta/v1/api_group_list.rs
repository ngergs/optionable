pub struct APIGroupListAc {
    pub groups: Option<
        <std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroup,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList {
    type Optioned = APIGroupListAc;
}
#[automatically_derived]
impl crate::Optionable for APIGroupListAc {
    type Optioned = APIGroupListAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIGroupList {
    fn into_optioned(self) -> APIGroupListAc {
        APIGroupListAc {
            groups: Some(crate::OptionableConvert::into_optioned(self.groups)),
        }
    }
    fn try_from_optioned(
        value: APIGroupListAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            groups: crate::OptionableConvert::try_from_optioned(
                value
                    .groups
                    .ok_or(crate::optionable::Error {
                        missing_field: "groups",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: APIGroupListAc) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.groups {
            crate::OptionableConvert::merge(&mut self.groups, other_value)?;
        }
        Ok(())
    }
}
