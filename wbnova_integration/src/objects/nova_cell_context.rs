use reqwest::{header, ClientBuilder};

#[derive(Debug, Clone)]
pub struct NovaCellContext {
    pub host: String,
    pub cell: String,
    pub access_token: String,
    pub secure_connection: bool,
}

impl NovaCellContext {
    pub fn from(host: String, cell: String, access_token: String, secure_connection: bool) -> Self {
        Self {
            host,
            cell,
            access_token,
            secure_connection,
        }
    }

    pub fn get_http_client(&self) -> ClientBuilder {
        let mut headers = header::HeaderMap::new();

        if self.access_token.len() > 0 {
            let auth_header =
                header::HeaderValue::from_str(&format!("Bearer {}", self.access_token))
                    .expect("Invalid auth header");
            headers.insert(header::AUTHORIZATION, auth_header);
        }

        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Godot"),
        );

        return reqwest::Client::builder().default_headers(headers);
    }

    pub fn get_http_base_url(&self) -> String {
        let protocol = match self.secure_connection {
            true => "https",
            _ => "http",
        };
        return format!("{protocol}://{}/api/v2", self.host);
    }

    pub fn get_websocket_base_url(&self) -> String {
        let protocol = match self.secure_connection {
            true => "wss",
            _ => "ws",
        };
        return format!("{protocol}://{}/api/v2", self.host);
    }

    pub fn get_websocket_headers(&self) -> Vec<(String, String)> {
        let mut headers = vec![(header::USER_AGENT.as_str().to_owned(), "Godot".to_owned())];

        if self.access_token.len() > 0 {
            headers.push((
                header::AUTHORIZATION.as_str().to_owned(),
                format!("Bearer {}", self.access_token),
            ));
        }
        headers
    }
}
