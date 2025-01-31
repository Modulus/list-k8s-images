use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, ListParams, ObjectList};
use kube::Client;

use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    namespace: Option<String>,
    // #[arg(short, long)]
    // label: String,
    #[arg(short, long, default_value = "plain")]
    output: Option<Output>,

    #[arg(short, long, default_value = "false")]
    verbose: bool
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Output {
    Plain,
    Json,
}
#[derive(Serialize, Deserialize)]
struct ContainerInfoCompact {
    name: String,
    image: String
}

#[derive(Serialize, Deserialize)]
struct PodInfoCompact {
    pod: String,
    containers: Vec<ContainerInfoCompact>
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

    match ns.list(&ListParams::default()).await {
        Ok(pods) => {
            if let Some(Output::Plain) = args.output {
                if args.verbose { 
                    eprintln!("Verbose mode not supported for plain output. Printing plain pods");
                  }

                print_plain_pods(pods).await;

            } else {
                if args.verbose {
                    println!("Verbose mode");
                    let pretty = serde_json::to_string_pretty(&pods).unwrap_or("Could not convert".into());
                    println!("{}", pretty);
                }
                else {
                    let compact = convert_pods_to_compact(pods).await;
                    let pretty = serde_json::to_string_pretty(&compact).unwrap_or("Could not convert".into());
                    println!("{}", pretty);
                }

            }
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }




    Ok(())
}


async fn convert_pods_to_compact(pods: ObjectList<Pod>) -> Vec<PodInfoCompact> {
    let mut pod_info_compact: Vec<PodInfoCompact> = Vec::new();

    for pod in pods {
        let mut container_info_compact: Vec<ContainerInfoCompact> = Vec::new();
        for container in pod.spec.unwrap().containers {
            container_info_compact.push(ContainerInfoCompact {
                name: container.name,
                image: container.image.unwrap()
            });
        }

        pod_info_compact.push(PodInfoCompact {
            pod: pod.metadata.name.unwrap(),
            containers: container_info_compact
        });
    }

    return pod_info_compact;
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
