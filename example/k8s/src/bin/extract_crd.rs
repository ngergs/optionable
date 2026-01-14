use kube::{Api, Client};
use optionable::kube3::ExtractManagedFields;
use optionable_k8s_example::{CustomCrd, FIELD_MANAGER};

// CRD definition is in ../lib.rs

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client: Client = Client::try_default().await?;
    let my_crd_api: Api<CustomCrd> = Api::default_namespaced(client);
    let my_cr = my_crd_api.get("test").await?;
    let my_cr_owned_fields = my_cr.extract(FIELD_MANAGER)?;
    println!("{my_cr_owned_fields:#?}");
    Ok(())
}
