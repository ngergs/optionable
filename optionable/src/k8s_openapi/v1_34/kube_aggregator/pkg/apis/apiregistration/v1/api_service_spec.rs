pub struct APIServiceSpecOpt {
    pub ca_bundle: <Option<::k8s_openapi::ByteString> as crate::Optionable>::Optioned,
    pub group: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub group_priority_minimum: Option<i32>,
    pub insecure_skip_tls_verify: <Option<bool> as crate::Optionable>::Optioned,
    pub service: <Option<
        ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference,
    > as crate::Optionable>::Optioned,
    pub version: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub version_priority: Option<i32>,
}
#[automatically_derived]
impl crate::Optionable
for ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceSpec {
    type Optioned = APIServiceSpecOpt;
}
#[automatically_derived]
impl crate::Optionable for APIServiceSpecOpt {
    type Optioned = APIServiceSpecOpt;
}
#[automatically_derived]
impl crate::OptionableConvert
for ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::APIServiceSpec {
    fn into_optioned(self) -> APIServiceSpecOpt {
        APIServiceSpecOpt {
            ca_bundle: <Option<
                ::k8s_openapi::ByteString,
            > as crate::OptionableConvert>::into_optioned(self.ca_bundle),
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.group),
            group_priority_minimum: Some(self.group_priority_minimum),
            insecure_skip_tls_verify: <Option<
                bool,
            > as crate::OptionableConvert>::into_optioned(self.insecure_skip_tls_verify),
            service: <Option<
                ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference,
            > as crate::OptionableConvert>::into_optioned(self.service),
            version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::into_optioned(self.version),
            version_priority: Some(self.version_priority),
        }
    }
    fn try_from_optioned(
        value: APIServiceSpecOpt,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            ca_bundle: <Option<
                ::k8s_openapi::ByteString,
            > as crate::OptionableConvert>::try_from_optioned(value.ca_bundle)?,
            group: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.group)?,
            group_priority_minimum: value
                .group_priority_minimum
                .ok_or(crate::optionable::Error {
                    missing_field: "group_priority_minimum",
                })?,
            insecure_skip_tls_verify: <Option<
                bool,
            > as crate::OptionableConvert>::try_from_optioned(
                value.insecure_skip_tls_verify,
            )?,
            service: <Option<
                ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference,
            > as crate::OptionableConvert>::try_from_optioned(value.service)?,
            version: <Option<
                std::string::String,
            > as crate::OptionableConvert>::try_from_optioned(value.version)?,
            version_priority: value
                .version_priority
                .ok_or(crate::optionable::Error {
                    missing_field: "version_priority",
                })?,
        })
    }
    fn merge(
        &mut self,
        other: APIServiceSpecOpt,
    ) -> Result<(), crate::optionable::Error> {
        <Option<
            ::k8s_openapi::ByteString,
        > as crate::OptionableConvert>::merge(&mut self.ca_bundle, other.ca_bundle)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.group, other.group)?;
        if let Some(other_value) = other.group_priority_minimum {
            self.group_priority_minimum = other_value;
        }
        <Option<
            bool,
        > as crate::OptionableConvert>::merge(
            &mut self.insecure_skip_tls_verify,
            other.insecure_skip_tls_verify,
        )?;
        <Option<
            ::k8s_openapi::kube_aggregator::pkg::apis::apiregistration::v1::ServiceReference,
        > as crate::OptionableConvert>::merge(&mut self.service, other.service)?;
        <Option<
            std::string::String,
        > as crate::OptionableConvert>::merge(&mut self.version, other.version)?;
        if let Some(other_value) = other.version_priority {
            self.version_priority = other_value;
        }
        Ok(())
    }
}
