use clap::Parser;

use aws_sdk_s3::{Client, Error};
use aws_config::BehaviorVersion;

mod services;

use services::create_files::create_files;

//https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/dotnetv3/S3/S3_Basics/S3Bucket.cs#L12
//https://github.com/awsdocs/aws-doc-sdk-examples/blob/main/rustv1/examples/s3/src/lib.rs

#[derive(Parser)]
struct Args {
    #[arg(long)]
    bucket_name_arg: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = create_files();
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

pub async fn create_bucket(
    client: &aws_sdk_s3::Client,
    bucket_name: &str,
) -> Result<Option<aws_sdk_s3::operation::create_bucket::CreateBucketOutput>, Error> {
    
    let create = client
        .create_bucket()
        .bucket(bucket_name)
        .send()
        .await;

    // BucketAlreadyExists and BucketAlreadyOwnedByYou are not problems for this task.
    create.map(Some).or_else(|err| {
        if err
            .as_service_error()
            .map(|se| se.is_bucket_already_exists() || se.is_bucket_already_owned_by_you())
            == Some(true)
        {
            Ok(None)
        } else {
            Err(Error::from(err))
        }
    })
}
