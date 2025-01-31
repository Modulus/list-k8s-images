use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, ListParams, ObjectList};
use kube::{client::ConfigExt, Client, Config};

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    namespace: Option<String>,
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


fn get_namespace(ns_arg : Option<String>, client: Client) -> String {
    if let Some(namespace) = ns_arg {
        return namespace;
    } else {
        return client.default_namespace().into();
    }
}

#[tokio::main]
async fn main() -> Result<(), kube::Error> {

    let args = Args::parse();

    let client = kube::Client::try_default().await?;

    let namespace = get_namespace(args.namespace, client.clone());
    println!("Namespace: {:?}", namespace);

    let ns: Api<Pod> = Api::namespaced(client.clone(), &namespace);
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



#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn get_namespace_has_option_set_returns_this_value() {

        let ns = Some("superawsome".to_string());
        let client = kube::Client::try_default().await.unwrap();
        let result = get_namespace(ns, client);
        assert_eq!("superawsome", result);
    }
}
