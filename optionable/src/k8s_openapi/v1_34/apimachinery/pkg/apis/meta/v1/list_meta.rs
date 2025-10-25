pub struct ListMetaOpt {
    pub continue_: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub remaining_item_count: <Option<i64> as crate::Optionable>::Optioned,
    pub resource_version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub self_link: <Option<std::string::String> as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta {
    type Optioned = ListMetaOpt;
}
#[automatically_derived]
impl crate::Optionable for ListMetaOpt {
    type Optioned = ListMetaOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta {
    fn into_optioned(self) -> ListMetaOpt {
        ListMetaOpt {
            continue_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.continue_),
            remaining_item_count: <Option<
                i64,
            > as crate::OptionableConvert>::into_optioned(self.remaining_item_count),
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.resource_version),
            self_link: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.self_link),
        }
    }
    fn try_from_optioned(value: ListMetaOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            continue_: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.continue_)?,
            remaining_item_count: <Option<
                i64,
            > as crate::OptionableConvert>::try_from_optioned(
                value.remaining_item_count,
            )?,
            resource_version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.resource_version)?,
            self_link: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.self_link)?,
        })
    }
    fn merge(&mut self, other: ListMetaOpt) -> Result<(), crate::optionable::Error> {
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.continue_, other.continue_)?;
        <Option<
            i64,
        > as crate::OptionableConvert>::merge(
            &mut self.remaining_item_count,
            other.remaining_item_count,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(
            &mut self.resource_version,
            other.resource_version,
        )?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.self_link, other.self_link)?;
        Ok(())
    }
}
