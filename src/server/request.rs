use crate::errors::ErrorRequest;
use reqwest::{Error, RequestBuilder, Response};
use std::time::Duration;
use tokio::time;

pub async fn send_request_with_exponential_backoff(
    request: &RequestBuilder,
) -> Result<Response, ErrorRequest> {
    let mut response = try_sending_request(&request).await;

    let mut number_of_retry_sending_request: u8 = 10;
    let mut backoff_duration = Duration::from_secs(1);
    while response.is_err() && number_of_retry_sending_request != 0 {
        number_of_retry_sending_request -= 1;
        time::sleep(backoff_duration).await;
        backoff_duration *= 2; // Exponential backoff
        response = try_sending_request(&request).await;
    }
    response.map_err(|_| ErrorRequest::ErrorSendingRequest)
}

async fn try_sending_request(request: &RequestBuilder) -> Result<Response, Error> {
    request.try_clone().expect("Error cloning").send().await
}
