with open("src/lib.rs", "r") as f:
    text = f.read()

old_str = """            if let Ok(content) = fs::read(&file_path) {
                let mime = mime_from_path(&path);
                let response = wry::http::Response::builder()
                    .header("Content-Type", mime)
                    .header("Access-Control-Allow-Origin", "*")
                    .body(Cow::Owned(content))
                    .unwrap();
                responder.respond(response);
            } else {"""

new_str = """            if let Ok(content) = fs::read(&file_path) {
                let mime = mime_from_path(&path);
                let mut builder = wry::http::Response::builder()
                    .header("Content-Type", mime.clone())
                    .header("Access-Control-Allow-Origin", "*");

                if mime == "text/html" {
                    builder = builder.header(
                        "Content-Security-Policy",
                        "default-src 'self' forge: http://localhost:*; \
                         script-src 'self' 'unsafe-inline' 'unsafe-eval' forge: http://localhost:*; \
                         style-src 'self' 'unsafe-inline' forge: http://localhost:*; \
                         img-src 'self' data: forge: http://localhost:*; \
                         connect-src 'self' ws://localhost:* http://localhost:* forge:;"
                    );
                }

                let response = builder.body(Cow::Owned(content)).unwrap();
                responder.respond(response);
            } else {"""

text = text.replace(old_str, new_str)
with open("src/lib.rs", "w") as f:
    f.write(text)
