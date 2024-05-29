use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use labrinth::auth::email::{send_email, send_email_raw};
use labrinth::routes::internal::flows::create_account_internal;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[actix_rt::test]
async fn send_email_test() {
    match send_email_raw(
        "wyd1542536763@gmail.com".to_string(),
        "test".to_string(),
        "test".to_string(),
    ) {
        Ok(my_result) => println!("Success"),
        Err(err) => println!("Error: {}", err),
    }
}

#[actix_rt::test]
async fn password_hash_test() {
    let hasher = Argon2::default();
    let salt = SaltString::generate(&mut ChaCha20Rng::from_entropy());
    let password_hash = hasher.hash_password("123456".as_bytes(), &salt);
    match password_hash {
        Ok(res) => {
            println!("hash: {}", res.to_string())
        }
        Err(e) => {
            println!("Error {}", e)
        }
    }
}
