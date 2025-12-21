use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::{Api, Client};
use optionable_k8s_example::{
    CustomCrd, CustomCrdAc, CustomCrdSpecAc, CustomCrdSpecTemplateAc, CustomCrdSpecTemplateType,
    FIELD_MANAGER,
};

// CRD definition is in ../lib.rs

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::try_default().await?;
    let my_crd_api: Api<CustomCrd> = Api::default_namespaced(client);

    let custom_cr_patch = CustomCrdAc {
        metadata: ObjectMeta {
            name: Some("test".to_owned()),
            ..Default::default()
        },
        spec: Some(CustomCrdSpecAc {
            template: Some(CustomCrdSpecTemplateAc {
                type_: Some(CustomCrdSpecTemplateType::V1beta1),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    my_crd_api
        .patch(
            custom_cr_patch
                .metadata
                .name
                .as_ref()
                .ok_or("name missing")?,
            &PatchParams::apply(FIELD_MANAGER).force(),
            &Patch::Apply(&custom_cr_patch),
        )
        .await?;
    Ok(())
}
