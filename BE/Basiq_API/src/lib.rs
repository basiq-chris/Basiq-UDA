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






// Data Structures
#[derive(Clone)]
pub struct Token {
    pub val: String,
    pub exp: u64
}

impl Token {
    pub fn get_token(x: Self) -> Self {
        if !Self::has_expired(&x) {
            return x
        } 
        let x: Token;
            let auth = Core::Authentication::gen_auth::get();
            x = Token {
                val: auth.0,
                exp: auth.1
            };
        x
        }

    fn has_expired(x: &Self) -> bool {
        if x.exp < SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() {
            return false
        }
        true
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