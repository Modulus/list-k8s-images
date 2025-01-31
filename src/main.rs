use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, ListParams, ObjectList};


use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    namespace: String,
    // #[arg(short, long)]
    // label: String,
    #[arg(short, long, default_value = "plain")]
    output: Option<Output>
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Output {
    Plain,
    Json,
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

    if let Some(Output::Plain) = args.output {
        print_plain_pods(pods).await;
    } else {
        let json = serde_json::to_string(&pods).unwrap();
        println!("{json}")
    }


    Ok(())
}


async fn print_plain_pods(pods: ObjectList<Pod>) {
    for pod in pods {
        println!("Pod: {:?}", pod.metadata.name.unwrap());

        for container in pod.spec.unwrap().containers {
            println!("\tContainer: {:?}", container.name);
            println!("\tImage: {:?}", container.image.unwrap());
        }
        println!("\n");
    }

    println!("\n");
}
