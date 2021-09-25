// Library for environment variables
use std::env;

// Token Issuer, in this case, me.
pub const TOKEN_ISSUER = "Plastic Lace by Goutham Krishna K V";

// CSRF Constants
pub const CSRF_SECRET: [u8; 32] = *b"lfwrsTAaVvl1I1X3ohoXNurQ5XSUVklP";
pub const CSRF_COOKIE_KEY: &str = "CSRF";
pub const AUTHORIZATION_KEY: &str = "Authorization";

// Backup JWT Token, in case ENV doesn't set it.
pub const JWT_BACKUP: &str = "qdplgwdgNmKoqoQwitrWRekj8xESxKU6Yc6KcGa3EKFVir6iI3m67qtcXBFLVZ3d";

pub fn secret_key() -> String {
    match env::var("secret_key") {
        Ok(env_secret_key) => env_secret_key,
        Err(_) => JWT_BACKUP.to_owned()
    }
}
