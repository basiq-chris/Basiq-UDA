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
        pub mod delete_consent;
    }
    pub mod AuthLinks {
        pub mod create_auth_link;
        pub mod delete_auth_link;
        pub mod retrieve_auth_link;
    }
    pub mod Jobs {
        pub mod retrieve_job;
        pub mod get_user_jobs;
        //pub mod create_mfa_response;
    }
}
