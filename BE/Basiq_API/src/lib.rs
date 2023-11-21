#[allow(non_snake_case)]
pub mod Core {
    pub mod Authentication {
        pub mod gen_auth;
    }
    pub mod Users {
        pub mod create_user;
        pub mod retrieve_user;
        pub mod update_user;
        pub mod delete_user;
    }
    pub mod Consents {
        pub mod retrieve_consents;
    }
}
