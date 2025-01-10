use k8s_openapi::api::{apps::v1::Deployment, core::v1::Pod};
use kube::api::{Api, ListParams, ObjectList};
#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = kube::Client::try_default().await?;

    let ns: Api<Pod> = Api::namespaced(client.clone(), "odm-test");
    let params = ListParams::default().labels("app.kubernetes.io/instance=odm-test");
    let pods :ObjectList<Pod> = ns.list(&params).await?;

    // println!("{:?}", pods);
    println!("Pods in the cluster in ns odm-test");
    for pod in pods {
        println!("{:?}", pod.metadata.name.unwrap());
    }

    println!("\n\n");

    let ns: Api<Deployment> = Api::namespaced(client.clone(), "odm-test");
    let params = ListParams::default().labels("release=odm-test");
    let deployments :ObjectList<Deployment> = ns.list(&params).await?;



    println!("Deploymens in the cluster in ns odm-test");
    for deploy in deployments {
        match deploy.metadata.name {
            Some(name)  => {
                println!("{:?}", name);
            }
            _ => {
                println!("No name found");
            },
        }
    }

    Ok(())
}
