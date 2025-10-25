pub struct APIResourceListOpt {
    pub group_version: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub resources: Option<
        <std::vec::Vec<
            ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResource,
        > as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList {
    type Optioned = APIResourceListOpt;
}
#[automatically_derived]
impl crate::Optionable for APIResourceListOpt {
    type Optioned = APIResourceListOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResourceList {
    fn into_optioned(self) -> APIResourceListOpt {
        APIResourceListOpt {
            group_version: Some(
                <std::string::String as crate::OptionableConvert>::into_optioned(
                    self.group_version,
                ),
            ),
            resources: Some(
                <std::vec::Vec<
                    ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResource,
                > as crate::OptionableConvert>::into_optioned(self.resources),
            ),
        }
    }
    fn try_from_optioned(
        value: APIResourceListOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            group_version: <std::string::String as crate::OptionableConvert>::try_from_optioned(
                value
                    .group_version
                    .ok_or(crate::optionable::Error {
                        missing_field: "group_version",
                    })?,
            )?,
            resources: <std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResource,
            > as crate::OptionableConvert>::try_from_optioned(
                value
                    .resources
                    .ok_or(crate::optionable::Error {
                        missing_field: "resources",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: APIResourceListOpt,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.group_version {
            <std::string::String as crate::OptionableConvert>::merge(
                &mut self.group_version,
                other_value,
            )?;
        }
        if let Some(other_value) = other.resources {
            <std::vec::Vec<
                ::k8s_openapi::apimachinery::pkg::apis::meta::v1::APIResource,
            > as crate::OptionableConvert>::merge(&mut self.resources, other_value)?;
        }
        Ok(())
    }
}
