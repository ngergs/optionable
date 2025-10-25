pub struct IngressOpt {
    pub metadata: Option<
        <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::Optionable>::Optioned,
    >,
    pub spec: <Option<
        ::k8s_openapi::api::networking::v1::IngressSpec,
    > as crate::Optionable>::Optioned,
    pub status: <Option<
        ::k8s_openapi::api::networking::v1::IngressStatus,
    > as crate::Optionable>::Optioned,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::Ingress {
    type Optioned = IngressOpt;
}
#[automatically_derived]
impl crate::Optionable for IngressOpt {
    type Optioned = IngressOpt;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::Ingress {
    fn into_optioned(self) -> IngressOpt {
        IngressOpt {
            metadata: Some(
                <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::into_optioned(
                    self.metadata,
                ),
            ),
            spec: <Option<
                ::k8s_openapi::api::networking::v1::IngressSpec,
            > as crate::OptionableConvert>::into_optioned(self.spec),
            status: <Option<
                ::k8s_openapi::api::networking::v1::IngressStatus,
            > as crate::OptionableConvert>::into_optioned(self.status),
        }
    }
    fn try_from_optioned(value: IngressOpt) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            metadata: <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::try_from_optioned(
                value
                    .metadata
                    .ok_or(crate::optionable::Error {
                        missing_field: "metadata",
                    })?,
            )?,
            spec: <Option<
                ::k8s_openapi::api::networking::v1::IngressSpec,
            > as crate::OptionableConvert>::try_from_optioned(value.spec)?,
            status: <Option<
                ::k8s_openapi::api::networking::v1::IngressStatus,
            > as crate::OptionableConvert>::try_from_optioned(value.status)?,
        })
    }
    fn merge(&mut self, other: IngressOpt) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.metadata {
            <::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as crate::OptionableConvert>::merge(
                &mut self.metadata,
                other_value,
            )?;
        }
        <Option<
            ::k8s_openapi::api::networking::v1::IngressSpec,
        > as crate::OptionableConvert>::merge(&mut self.spec, other.spec)?;
        <Option<
            ::k8s_openapi::api::networking::v1::IngressStatus,
        > as crate::OptionableConvert>::merge(&mut self.status, other.status)?;
        Ok(())
    }
}
