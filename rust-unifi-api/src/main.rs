use curl::easy::Easy;
use curl::easy::HttpVersion;

struct Client {
    baseurl: String,
    user: String,
    password: String,
    site: String,
    version: String,
    debug: bool,
    is_logged_in: bool,
    is_unifi_os: bool,
    exec_retries: u32,
    cookies: String,
    last_results_raw: Option<String>,
    last_error_message: String,
    curl_ssl_verify_peer: bool,
    curl_ssl_verify_host: bool,
    curl_http_version: curl::easy::HttpVersion,
    curl_headers: Vec<String>,
    curl_method: String,
    curl_methods_allowed: Vec<String>,
    curl_request_timeout: u64,
    curl_connect_timeout: u64,
}

fn main() {
    let mut client = Client {
        baseurl: String::from("https://127.0.0.1:8443"),
        user: String::new(),
        password: String::new(),
        site: String::from("default"),
        version: String::from("6.2.26"),
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
        curl_method: String::from("GET"),
        curl_methods_allowed: vec![
            String::from("GET"),
            String::from("POST"),
            String::from("PUT"),
            String::from("DELETE"),
            String::from("PATCH"),
        ],
        curl_request_timeout: 30,
        curl_connect_timeout: 10,
    };
}
