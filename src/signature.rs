use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;
use url::form_urlencoded;

pub fn sign(
    method: &'static str,
    base_url: &str,
    path: &str,
    data: &HashMap<String, String>,
    sk: &str,
) -> String {
    let mut pars = Vec::new();
    for (k, v) in data.into_iter() {
        let data = form_urlencoded::Serializer::new(String::new())
            .append_pair(k, v)
            .finish();
        pars.push(data);
    }
    pars.sort();
    let mut p: String = pars.join("&");
    let meta = vec![
        method.to_string(),
        base_url.to_string(),
        path.to_string(),
        p.clone(),
    ]
    .join("\n");
    let mut sha256 = Hmac::<Sha256>::new(sk.as_bytes()).unwrap();
    sha256.input(meta.as_bytes());
    let osig = sha256.result().code();
    let signature = base64::encode(&osig);
    let sig_encode = form_urlencoded::Serializer::new(String::new())
        .append_pair("Signature", &signature)
        .finish();
    p += "&";
    p += &sig_encode;
    p
}
