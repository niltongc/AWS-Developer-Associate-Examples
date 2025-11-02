use clap::Parser;

use aws_sdk_s3::{Client};
use aws_config::BehaviorVersion;

mod services;

use services::create_files::create_files;
use services::create_bucket::create_bucket;

//https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/dotnetv3/S3/S3_Basics/S3Bucket.cs#L12
//https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/rustv1/examples/s3/src/lib.rs

// Usage cargo run -- --bucket-name-arg my-bucket-name

#[derive(Parser)]
struct Args {
    #[arg(long)]
    bucket_name_arg: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    if let Err(e) = create_files() {
        eprintln!("⚠️ Erro to create files: {}", e);
    }

    let args = Args::parse();
    let bucket_name = &args.bucket_name_arg;

    // Load AWS CLI
    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let region = shared_config.region().unwrap();
    println!("🌍 region load: {}", region);

    let client = Client::new(&shared_config);

    // Create bucket
    create_bucket(&client, bucket_name).await?;

    println!("✅ Bucket created: {}", bucket_name);
    Ok(())
}


