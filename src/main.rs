use lambda_http::{run, service_fn, Body, Request, RequestExt, Response};
use lambda_http::Error as LError;
use aws_sdk_s3::presigning::config::PresigningConfig;
use aws_sdk_s3::Client;
use std::time::Duration;
use uuid::Uuid;
use std::env;

// Get query
// Load table
// Perform query
// Upload results to S3
// Get object pre-signed URI for upload
// return 302 http code with pre-signed url in the Location header.
async fn process_query(
    query: &str,
) -> Result<Response<Body>, LError> {
    // Get bucket name from ENV
    let bucket = env::var("WORKING_BUCKET").unwrap();
    let expires_in = env::var("PRESIGN_EXPIRES_IN").unwrap().parse::<u64>().unwrap();

    // Setup S3-Client
    // No extra configuration is needed as long as your Lambda has
    // the necessary permissions attached to its role.
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // Set pre-signed url expiration
    let expires_in = Duration::from_secs(expires_in);

    // TODO: Perform query execution here
    // data-lake stuff
    let query_data = format!("raw serialized query response from: {}", query);

    // TODO: Generate uuid for result payload
    let object: String = Uuid::new_v4().to_string();

    // TODO: Upload results to S3 using generated object key
    let _ = client
        .put_object()
        .bucket(bucket.clone().as_str())
        .body(query_data.as_bytes().to_owned().into())
        .key(object.clone().as_str())
        .content_type("text/plain")
        .send()
        .await?;
    // TODO: Sort error handling for this PutObject operation.
    // ...
    //         .await
    //         .map_err(|err| {
    //             // In case of failure, log a detailed error to CloudWatch.
    //             error!(
    //                 "failed to upload file '{}' to S3 with error: {}",
    //                 &filename, err
    //             );
    //             // The sender of the request receives this message in response.
    //             FailureResponse {
    //                 body: "The lambda encountered an error and your message was not saved".to_owned(),
    //             }
    //         })?;
    // ...

    // Get pre-signed url for given object
    let presigned_request = client
        .get_object()
        .bucket(bucket.clone().as_str())
        .key(object.clone().as_str())
        .presigned(PresigningConfig::expires_in(expires_in)?)
        .await?;

    let resp = Response::builder()
        .status(302)
        .header("Location", presigned_request.uri().to_string())
        .body(presigned_request.uri().to_string().into());
    Ok(resp.unwrap())
}

/// This is the main body for the function.
async fn function_handler(event: Request) -> Result<Response<Body>, LError> {
    // Match on the `q` url query-parameters for an &str value
    Ok(match event.query_string_parameters().first("q") {
        Some(query) => process_query(query).await.unwrap(),
        _ => Response::builder()
            .status(400)
            .body("Empty query".into())
            .expect("failed to render response"),
    })
}

#[tokio::main]
async fn main() -> Result<(), LError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
