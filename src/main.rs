use std::env;

use anyhow::Error;

mod docker;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 1);
    println!("last words for {:?}", args);

    crate::docker::get_full_container_name(args[0].clone(),5000).await.unwrap(); 

    Ok(())
}
