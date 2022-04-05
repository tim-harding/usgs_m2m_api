pub trait Endpoint {
    const PATH: &'static str;
    type Parameters;
    type RawResponse;
    type Result;
}