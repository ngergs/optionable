#[derive(
    Clone,
    Default,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    std::fmt::Debug
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CustomResourceSubresourceScaleAc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_selector_path: <Option<
        std::string::String,
    > as crate::Optionable>::Optioned,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_replicas_path: Option<<std::string::String as crate::Optionable>::Optioned>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_replicas_path: Option<
        <std::string::String as crate::Optionable>::Optioned,
    >,
}
#[automatically_derived]
impl crate::Optionable
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale {
    type Optioned = CustomResourceSubresourceScaleAc;
}
#[automatically_derived]
impl crate::Optionable for CustomResourceSubresourceScaleAc {
    type Optioned = CustomResourceSubresourceScaleAc;
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionableConvert
for k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale {
    fn into_optioned(self) -> CustomResourceSubresourceScaleAc {
        CustomResourceSubresourceScaleAc {
            label_selector_path: crate::OptionableConvert::into_optioned(
                self.label_selector_path,
            ),
            spec_replicas_path: Some(
                crate::OptionableConvert::into_optioned(self.spec_replicas_path),
            ),
            status_replicas_path: Some(
                crate::OptionableConvert::into_optioned(self.status_replicas_path),
            ),
        }
    }
    fn try_from_optioned(
        value: CustomResourceSubresourceScaleAc,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            label_selector_path: crate::OptionableConvert::try_from_optioned(
                value.label_selector_path,
            )?,
            spec_replicas_path: crate::OptionableConvert::try_from_optioned(
                value
                    .spec_replicas_path
                    .ok_or(crate::Error {
                        missing_field: "spec_replicas_path",
                    })?,
            )?,
            status_replicas_path: crate::OptionableConvert::try_from_optioned(
                value
                    .status_replicas_path
                    .ok_or(crate::Error {
                        missing_field: "status_replicas_path",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: CustomResourceSubresourceScaleAc,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(
            &mut self.label_selector_path,
            other.label_selector_path,
        )?;
        if let Some(other_value) = other.spec_replicas_path {
            crate::OptionableConvert::merge(&mut self.spec_replicas_path, other_value)?;
        }
        if let Some(other_value) = other.status_replicas_path {
            crate::OptionableConvert::merge(
                &mut self.status_replicas_path,
                other_value,
            )?;
        }
        Ok(())
    }
}
#[automatically_derived]
#[cfg(feature = "k8s_openapi_convert")]
impl crate::OptionedConvert<
    k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale,
> for CustomResourceSubresourceScaleAc {
    fn from_optionable(
        value: k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale,
    ) -> Self {
        crate::OptionableConvert::into_optioned(value)
    }
    fn try_into_optionable(
        self,
    ) -> Result<
        k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale,
        crate::Error,
    > {
        crate::OptionableConvert::try_from_optioned(self)
    }
    fn merge_into(
        self,
        other: &mut k8s_openapi027::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceSubresourceScale,
    ) -> Result<(), crate::Error> {
        crate::OptionableConvert::merge(other, self)
    }
}
