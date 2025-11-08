use aws_sdk_s3::{Client};
use aws_sdk_s3::primitives::ByteStream;
use std::path::Path;

pub async fn upload_object(
    client: &Client,
    bucket_name: &str,
    file_names: &[&str],
) -> Result<Vec<aws_sdk_s3::operation::put_object::PutObjectOutput>, Box<dyn std::error::Error>> {

    let mut results = Vec::new();

    for &file_name in file_names {
        // converte ByteStreamError para Box<dyn Error>
        let body = ByteStream::from_path(Path::new(file_name)).await?;

        let result = client
            .put_object()
            .bucket(bucket_name)
            .key(file_name)
            .body(body)
            .send()
            .await?;

        results.push(result);
    }

    Ok(results)
}
