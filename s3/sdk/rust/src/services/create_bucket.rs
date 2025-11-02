use aws_sdk_s3::{Error};

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