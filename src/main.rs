use k8s_openapi::api::{apps::v1::Deployment, core::v1::Pod};
use kube::{api::{Api, ListParams, ObjectList}, core::params};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    namespace: String,
    // #[arg(short, long)]
    // label: String,
}

#[tokio::main]
async fn main() -> Result<(), kube::Error> {

    let args = Args::parse();

    println!("Namespace: {:?}", args.namespace);

    let client = kube::Client::try_default().await?;

    let ns: Api<Pod> = Api::namespaced(client.clone(), &args.namespace);
    // let params = ListParams::default().labels("app.kubernetes.io/instance=odm-test");
    let params = ListParams::default();
    let pods :ObjectList<Pod> = ns.list(&params).await?;

    // println!("{:?}", pods);
    println!("Pods in the cluster in ns {}", args.namespace);
    for pod in pods {
        println!("==========================");
        println!("Details for pod:");
        println!("{:?}", pod.metadata.name.unwrap());

        for container in pod.spec.unwrap().containers {
            println!("Container: {:?}", container.name);
            println!("Image: {:?}", container.image.unwrap());
        }

        println!("==========================\n\n");
    }

    println!("\n\n");


    Ok(())
}
