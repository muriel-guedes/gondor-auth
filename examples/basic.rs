use std::sync::Arc;

use gondor_auth::{GoogleOAuth2, google::GoogleOAuth2Builder};
use gondor_io::Method;

pub const ADDRESS: &'static str = env!("ADDRESS");

struct Context {
    gauth: GoogleOAuth2
}

fn main() {
    println!("Server running on http://{}/", ADDRESS);
    gondor_io::GondorIO::new(ADDRESS, Arc::new(Context {
        gauth: GoogleOAuth2Builder::new(
            env!("GOOGLE_CLIENT_ID"),
            env!("GOOGLE_CLIENT_SECRET"),
            format!("http://{ADDRESS}/goauth2/")
        ).add_scope(gondor_auth::google::scopes::OPENID)
        .build()
    }), |context, mut req| {
        let path = req.path();
        println!("path: {}", String::from_utf8_lossy(path));
        match req.method() {
            Method::GET => {
                if path == b"" {
                    let url = context.gauth.get_login_url();
                    req.send(200, "text/html", format!("goto: <a href=\"{}\">link</a>", url)).unwrap()
                } else {
                    req.send(200, "text/plain", "OK").unwrap()
                }
            },
            _ => {}
        }
    }, |_, e| {
        match e.kind() {
            std::io::ErrorKind::WouldBlock => {}, // Ignore non blocking errors
            _ => eprintln!("Error: {}", e)
        }
    }).unwrap().start_nonblocking();
}