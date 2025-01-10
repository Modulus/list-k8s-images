use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, ListParams, ObjectList};
#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = kube::Client::try_default().await?;

    let pods: Api<Pod> = Api::namespaced(client, "odm-test");
    let params = ListParams::default().labels("app.kubernetes.io/instance=odm-test");
    let pods :ObjectList<Pod> = pods.list(&params).await?;

    println!("{:?}", pods);


    for pod in pods {
        println!("{:?}", pod.metadata.name.unwrap());
    }

    Ok(())
}
