pub mod incircle {
    pub mod request {
        include!(concat!(env!("OUT_DIR"), "/incircle.request.rs"));
    }
    pub mod response {
        include!(concat!(env!("OUT_DIR"), "/incircle.response.rs"));
    }
}

pub use incircle::request;
pub use incircle::response;
pub use incircle::request::Request;
pub use incircle::response::Response;
pub use prost;
