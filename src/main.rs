#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use curl::easy::HttpVersion;
use macaddr;

enum CurlMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

struct VersionCode {
    major: u32,
    moder: u32,
    minor: u32,
}

impl VersionCode {
    pub fn from(code: &str) -> Self {
        let mut numr: Result<Vec<u32>, _> = code.split(".").map(|x| x.parse()).collect();
        let mut num = numr.unwrap();

        assert_eq!(num.len(), 3);
        Self {
            major: num[0],
            moder: num[1],
            minor: num[2],
        }
    }
}

struct UnifiClient {
    baseurl: String,
    user: String,
    password: String,
    site: String,
    version: VersionCode,
    debug: bool,
    is_logged_in: bool,
    is_unifi_os: bool,
    exec_retries: u64,
    cookies: String,
    last_results_raw: Option<String>,
    last_error_message: String,
    curl_ssl_verify_peer: bool,
    curl_ssl_verify_host: bool,
    curl_http_version: curl::easy::HttpVersion,
    curl_headers: Vec<String>,
    curl_method: CurlMethod,
    curl_methods_allowed: Vec<CurlMethod>,
    curl_request_timeout: u64,
    curl_connect_timeout: u64,
}

impl UnifiClient {
    pub fn new() -> Self {
        UnifiClient {
            baseurl: String::from("https://127.0.0.1:8443"),
            user: String::new(),
            password: String::new(),
            site: String::from("default"),
            version: VersionCode::from("6.2.26"),
            debug: false,
            is_logged_in: false,
            is_unifi_os: false,
            exec_retries: 0,
            cookies: String::new(),
            last_results_raw: None,
            last_error_message: String::new(),
            curl_ssl_verify_peer: false,
            curl_ssl_verify_host: false,
            curl_http_version: HttpVersion::Any,
            curl_headers: Vec::new(),
            curl_method: CurlMethod::GET,
            curl_methods_allowed: vec![
                CurlMethod::PUT,
                CurlMethod::PATCH,
                CurlMethod::DELETE,
                CurlMethod::POST,
                CurlMethod::GET,
            ],
            curl_request_timeout: 30,
            curl_connect_timeout: 10,
        }
    }

    // generic boilerplate
    // -------------------------------------------
    fn baseurl(&self) -> &str {
        self.baseurl.as_ref()
    }

    fn set_baseurl(&mut self, baseurl: String) {
        self.baseurl = baseurl;
    }

    fn user(&self) -> &str {
        self.user.as_ref()
    }

    fn set_user(&mut self, user: String) {
        self.user = user;
    }
    // -------------------------------------------
}

fn main() {
    let mut client: UnifiClient = UnifiClient::new();
}
