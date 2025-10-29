pub struct ResourceClaimConsumerReferenceOpt {
    pub api_group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub name: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub resource: Option<<std::string::String as crate::Optionable>::Optioned>,
    pub uid: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::api::resource::v1beta2::ResourceClaimConsumerReference {
    type Optioned = ResourceClaimConsumerReferenceOpt;
}
#[automatically_derived]
impl crate::Optionable for ResourceClaimConsumerReferenceOpt {
    type Optioned = ResourceClaimConsumerReferenceOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::resource::v1beta2::ResourceClaimConsumerReference {
    fn into_optioned(self) -> ResourceClaimConsumerReferenceOpt {
        ResourceClaimConsumerReferenceOpt {
            api_group: crate::OptionableConvert::into_optioned(self.api_group),
            name: Some(crate::OptionableConvert::into_optioned(self.name)),
            resource: Some(crate::OptionableConvert::into_optioned(self.resource)),
            uid: Some(crate::OptionableConvert::into_optioned(self.uid)),
        }
    }
    fn try_from_optioned(
        value: ResourceClaimConsumerReferenceOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            api_group: crate::OptionableConvert::try_from_optioned(value.api_group)?,
            name: crate::OptionableConvert::try_from_optioned(
                value
                    .name
                    .ok_or(crate::optionable::Error {
                        missing_field: "name",
                    })?,
            )?,
            resource: crate::OptionableConvert::try_from_optioned(
                value
                    .resource
                    .ok_or(crate::optionable::Error {
                        missing_field: "resource",
                    })?,
            )?,
            uid: crate::OptionableConvert::try_from_optioned(
                value
                    .uid
                    .ok_or(crate::optionable::Error {
                        missing_field: "uid",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceClaimConsumerReferenceOpt,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.api_group, other.api_group)?;
        if let Some(other_value) = other.name {
            crate::OptionableConvert::merge(&mut self.name, other_value)?;
        }
        if let Some(other_value) = other.resource {
            crate::OptionableConvert::merge(&mut self.resource, other_value)?;
        }
        if let Some(other_value) = other.uid {
            crate::OptionableConvert::merge(&mut self.uid, other_value)?;
        }
        Ok(())
    }
}
