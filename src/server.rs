#![allow(clippy::manual_strip)]
use std::io::stdout;

use rouille::Response;

use crate::config::Config;
use crate::store;

pub fn start(config: Config) {
    println!("Now listening on localhost:8321");
    let static_key: Vec<u8> = config.static_key;

    // https://github.com/tomaka/rouille/blob/c86bbf422d1d6f7a4798c762018a4b7922bb94b5/examples/login-session.rs
    // https://github.com/tomaka/rouille/blob/c86bbf422d1d6f7a4798c762018a4b7922bb94b5/examples/simple-form.rs
    rouille::start_server("localhost:8321", move |request| {
        rouille::log(&request, stdout(), || {
            router!(request,
                (GET) (/) => {
                    if let Some(k) = request.get_param("k") {
                        let resp = match store::retrieve(&k, &static_key) {
                            Some(message) => MSG.replace("{{__MESSAGE__}}", &message),
                            None => MSG.replace("{{__MESSAGE__}}", "Secret not found!"),
                        };
                        rouille::Response::html(resp)
                    } else {
                        rouille::Response::html(FORM)
                    }
                },
                (POST) (/submit) => {
                    let data = try_or_400!(post_input!(request, {
                        txt: String,
                    }));

                    let k = store::save(data.txt.as_bytes().to_vec(), &static_key);
                    let resp = SBMT.replace("{{__K__}}", &k);
                    rouille::Response::html(resp)
                },
                _ => {
                    let response = rouille::match_assets(&request, "./static");
                    if response.is_success() {
                        response
                    } else {
                        Response::html(FOUR_O_FOUR)
                    }
                }
            )
        })
    });
}

static FOUR_O_FOUR: &str = r#"
<html>
    <head>
        <title>flashpaper-rs</title>
    </head>
    <body style="margin: 1%">
        404
    </body>
</html>
"#;

static FORM: &str = include_str!("../static/index.html");

static SBMT: &str = r#"
<html>
    <head>
        <title>flashpaper-rs</title>
    </head>
    <body style="margin: 1%">
        <h1>flashpaper-rs</h1>
        <div style="text-align: center;">
            <p>Success!</p>
            <p>Your url is http://localhost:8321/?k={{__K__}}</p>
            <a href="/">Home</a>
        </div>
    </body>
    </html>
"#;

static MSG: &str = r#"
<html>
    <head>
        <title>flashpaper-rs</title>
    </head>
    <body>
        <h1>flashpaper-rs</h1>
        <div style="text-align: center;">
            <p>{{__MESSAGE__}}</p>
            <a href="/">Home</a>
        </div>
    </body>
</html>
"#;
