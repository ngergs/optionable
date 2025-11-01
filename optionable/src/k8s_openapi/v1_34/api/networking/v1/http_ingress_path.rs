pub struct HTTPIngressPathAc {
    pub backend: Option<
        <::k8s_openapi::api::networking::v1::IngressBackend as crate::Optionable>::Optioned,
    >,
    pub path: <Option<std::string::String> as crate::Optionable>::Optioned,
    pub path_type: Option<<std::string::String as crate::Optionable>::Optioned>,
}
#[automatically_derived]
impl crate::Optionable for ::k8s_openapi::api::networking::v1::HTTPIngressPath {
    type Optioned = HTTPIngressPathAc;
}
#[automatically_derived]
impl crate::Optionable for HTTPIngressPathAc {
    type Optioned = HTTPIngressPathAc;
}
#[automatically_derived]
impl crate::OptionableConvert for ::k8s_openapi::api::networking::v1::HTTPIngressPath {
    fn into_optioned(self) -> HTTPIngressPathAc {
        HTTPIngressPathAc {
            backend: Some(crate::OptionableConvert::into_optioned(self.backend)),
            path: crate::OptionableConvert::into_optioned(self.path),
            path_type: Some(crate::OptionableConvert::into_optioned(self.path_type)),
        }
    }
    fn try_from_optioned(
        value: HTTPIngressPathAc,
    ) -> Result<Self, crate::optionable::Error> {
        Ok(Self {
            backend: crate::OptionableConvert::try_from_optioned(
                value
                    .backend
                    .ok_or(crate::optionable::Error {
                        missing_field: "backend",
                    })?,
            )?,
            path: crate::OptionableConvert::try_from_optioned(value.path)?,
            path_type: crate::OptionableConvert::try_from_optioned(
                value
                    .path_type
                    .ok_or(crate::optionable::Error {
                        missing_field: "path_type",
                    })?,
            )?,
        })
    }
    fn merge(
        &mut self,
        other: HTTPIngressPathAc,
    ) -> Result<(), crate::optionable::Error> {
        if let Some(other_value) = other.backend {
            crate::OptionableConvert::merge(&mut self.backend, other_value)?;
        }
        crate::OptionableConvert::merge(&mut self.path, other.path)?;
        if let Some(other_value) = other.path_type {
            crate::OptionableConvert::merge(&mut self.path_type, other_value)?;
        }
        Ok(())
    }
}
