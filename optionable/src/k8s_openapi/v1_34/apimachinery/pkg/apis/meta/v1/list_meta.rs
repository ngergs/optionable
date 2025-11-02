#[derive(Clone, std::fmt::Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ListMetaAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_item_count: <Option<i64> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_link: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta {
    type Optioned = ListMetaAc;
}
#[automatically_derived]
impl crate::Optionable for ListMetaAc {
    type Optioned = ListMetaAc;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta {
    fn into_optioned(self) -> ListMetaAc {
        ListMetaAc {
            continue_: crate::OptionableConvert::into_optioned(self.continue_),
            remaining_item_count: crate::OptionableConvert::into_optioned(
                self.remaining_item_count,
            ),
            resource_version: crate::OptionableConvert::into_optioned(
                self.resource_version,
            ),
            self_link: crate::OptionableConvert::into_optioned(self.self_link),
        }
    }
    fn try_from_optioned(value: ListMetaAc) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            continue_: crate::OptionableConvert::try_from_optioned(value.continue_)?,
            remaining_item_count: crate::OptionableConvert::try_from_optioned(
                value.remaining_item_count,
            )?,
            resource_version: crate::OptionableConvert::try_from_optioned(
                value.resource_version,
            )?,
            self_link: crate::OptionableConvert::try_from_optioned(value.self_link)?,
        })
    }
    fn merge(&mut self, other: ListMetaAc) -> Result<(), crate::optionable::Error> {
        crate::OptionableConvert::merge(&mut self.continue_, other.continue_)?;
        crate::OptionableConvert::merge(
            &mut self.remaining_item_count,
            other.remaining_item_count,
        )?;
        crate::OptionableConvert::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        crate::OptionableConvert::merge(&mut self.self_link, other.self_link)?;
        Ok(())
    }
}
