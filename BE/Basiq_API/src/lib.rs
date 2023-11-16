use std::time::{SystemTime, UNIX_EPOCH};

#[allow(non_snake_case)]
pub mod Core {
    pub mod Authentication {
        pub mod gen_auth;
    }
    pub mod Users {
        pub mod create_user;
    }
}

#[derive(Default)]
pub struct User {
   pub first_name: Option<String>,
   pub middle_name: Option<String>,
   pub last_name: Option<String>,
   pub email: Option<String>,
   pub phone_number: Option<String>
}