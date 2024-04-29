use labrinth::auth::email::{send_email, send_email_raw};
use labrinth::routes::internal::flows::create_account_internal;

#[actix_rt::test]
async fn send_email_test() {
    match send_email_raw("wyd1542536763@gmail.com".to_string(), "test".to_string(), "test".to_string()) {
        Ok(my_result) => println!("Success"),
        Err(err) => println!("Error: {}", err),
    }
}