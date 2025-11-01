pub struct ListAc<T>
where
    T: k8s_openapi::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized,
{
    pub items: Option<<std::vec::Vec<T> as crate::Optionable>::Optioned>,
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ListMeta as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl<T> crate::Optionable for ::k8s_openapi::List<T>
where
    T: k8s_openapi::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized,
{
    type Optioned = ListAc<T>;
}
#[automatically_derived]
impl<T> crate::Optionable for ListAc<T>
where
    T: k8s_openapi::ListableResource + crate::Optionable,
    <T as crate::Optionable>::Optioned: Sized,
{
    type Optioned = ListAc<T>;
}
#[automatically_derived]
impl<T> crate::OptionableConvert for ::k8s_openapi::List<T>
where
    T: k8s_openapi::ListableResource + crate::OptionableConvert,
    <T as crate::Optionable>::Optioned: Sized,
{
    fn into_optioned(self) -> ListAc<T> {
        ListAc::<T> {
            items: Some(crate::OptionableConvert::into_optioned(self.items)),
            metadata: Some(crate::OptionableConvert::into_optioned(self.metadata)),
        }
    }
    fn try_from_optioned(value: ListAc<T>) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            items: crate::OptionableConvert::try_from_optioned(
                value
                    .items
                    .ok_or(crate::optionable::Error {
                        missing_field: "items",
                    })?,
            )?,
            metadata: crate::OptionableConvert::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
        })
    }
    fn merge(&mut self, other: ListAc<T>) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.items {
            crate::OptionableConvert::merge(&mut self.items, other_value)?;
        }
        if let Some(other_value) = other.metadata {
            crate::OptionableConvert::merge(&mut self.metadata, other_value)?;
        }
        Ok(())
    }
}
