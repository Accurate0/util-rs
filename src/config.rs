use config::Config;

use crate::error::Error;

pub async fn load_from_s3<'a, T: serde::Deserialize<'a>>(
    shared_config: &aws_types::SdkConfig,
    bucket_name: &str,
    filename: &str,
) -> Result<T, Error> {
    let s3_client = aws_sdk_s3::Client::new(&shared_config);

    let resp = s3_client
        .get_object()
        .bucket(bucket_name)
        .key(filename)
        .send()
        .await?;

    let config = resp.body.collect().await?;
    Ok(Config::builder()
        .add_source(config::File::from_str(
            std::str::from_utf8(&config.clone().into_bytes())?,
            config::FileFormat::Json,
        ))
        .build()?
        .try_deserialize::<T>()?)
}
