#[derive(
    Clone,
    std::fmt::Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAttributesAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::authorization::v1::ResourceAttributes {
    type Optioned = ResourceAttributesAc;
}
#[automatically_derived]
impl crate::Optionable for ResourceAttributesAc {
    type Optioned = ResourceAttributesAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::api::authorization::v1::ResourceAttributes {
    fn into_optioned(self) -> ResourceAttributesAc {
        ResourceAttributesAc {
            group: crate::OptionableConvert::into_optioned(self.group),
            name: crate::OptionableConvert::into_optioned(self.name),
            namespace: crate::OptionableConvert::into_optioned(self.namespace),
            resource: crate::OptionableConvert::into_optioned(self.resource),
            subresource: crate::OptionableConvert::into_optioned(self.subresource),
            verb: crate::OptionableConvert::into_optioned(self.verb),
            version: crate::OptionableConvert::into_optioned(self.version),
        }
    }
    fn try_from_optioned(
        value: ResourceAttributesAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            group: crate::OptionableConvert::try_from_optioned(value.group)?,
            name: crate::OptionableConvert::try_from_optioned(value.name)?,
            namespace: crate::OptionableConvert::try_from_optioned(value.namespace)?,
            resource: crate::OptionableConvert::try_from_optioned(value.resource)?,
            subresource: crate::OptionableConvert::try_from_optioned(value.subresource)?,
            verb: crate::OptionableConvert::try_from_optioned(value.verb)?,
            version: crate::OptionableConvert::try_from_optioned(value.version)?,
        })
    }
    fn merge(
        &mut self,
        other: ResourceAttributesAc,
    ) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.group, other.group)?;
        crate::OptionableConvert::merge(&mut self.name, other.name)?;
        crate::OptionableConvert::merge(&mut self.namespace, other.namespace)?;
        crate::OptionableConvert::merge(&mut self.resource, other.resource)?;
        crate::OptionableConvert::merge(&mut self.subresource, other.subresource)?;
        crate::OptionableConvert::merge(&mut self.verb, other.verb)?;
        crate::OptionableConvert::merge(&mut self.version, other.version)?;
        Ok(())
    }
}
