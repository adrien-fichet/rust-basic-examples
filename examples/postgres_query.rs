/*
Setup a local Postgres on k8s (k3d cluster):

k3d cluster create demo --image rancher/k3s:v1.32.0-k3s1 -p "5432:30000@agent:0" --agents 1
export POSTGRES_PASSWORD=xxx
helm install postgresql -n postgresql --create-namespace \
  --set primary.service.nodePorts.postgresql=30000 \
  --set primary.service.type=NodePort \
  --set auth.database=demo \
  --set auth.postgresPassword=$POSTGRES_PASSWORD \
  oci://registry-1.docker.io/bitnamicharts/postgresql
kubectl rollout status sts/postgresql -n postgresql
cargo run --example postgres_query
*/

use itertools::Itertools;
use std::env;
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host=localhost user=postgres password={} dbname=demo",
            env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set")
        ),
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Postgres connection error: {}", e);
        }
    });

    let client = &client;

    client
        .query(
            "CREATE TABLE IF NOT EXISTS cats (id SERIAL PRIMARY KEY, name TEXT)",
            &[],
        )
        .await?;

    client
        .query("INSERT INTO cats (name) VALUES ($1)", &[&"Riki"])
        .await?;

    let rows = client.query("SELECT id, name FROM cats", &[]).await?;
    println!(
        "{}",
        rows.iter()
            .map(|row| {
                let id: i32 = row.get(0);
                let name: &str = row.get(1);
                format!("{}: {}", id, name)
            })
            .format("\n")
    );
    println!("Got {} cats", rows.len());

    let cat_name: &str = rows[0].get(1);
    println!("First cat name is {}", cat_name);

    todo!("Connection over TLS");
    //Ok(())
}
