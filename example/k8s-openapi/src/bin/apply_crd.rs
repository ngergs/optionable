use kube::api::{ObjectMeta, Patch, PatchParams};
use kube::{Api, Client};
use optionable_k8s_openapi_example::{MyCrd, MyCrdAc, MyCrdSpecAc, MyCrdSpecTemplateAc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::try_default().await?;
    let my_crd: Api<MyCrd> = Api::default_namespaced(client);
    let patch = MyCrdAc {
        metadata: ObjectMeta {
            name: Some("test".to_owned()),
            ..Default::default()
        },
        spec: Some(MyCrdSpecAc {
            template: Some(MyCrdSpecTemplateAc { replicas: Some(2) }),
            ..Default::default()
        }),
        ..Default::default()
    };
    my_crd
        .patch(
            &patch.metadata.name.clone().ok_or("name missing")?,
            &PatchParams::apply("rust-manager").force(),
            &Patch::Apply(patch),
        )
        .await?;
    Ok(())
}
