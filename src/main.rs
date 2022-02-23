extern crate dioxus;
extern crate axum;

use axum::{routing::get, Router};
use dioxus::prelude::*;
use std::net::SocketAddr;
use dioxus::prelude::Scope;
use axum::response::Html;


#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(home))
        .route("/signin", get(login))
        .route("/register", get(register));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("- Route available on http://{}", addr);
    println!("- Route available on http://{}/signin", addr);
    println!("- Route available on http://{}/register", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn register() -> Html<String> {

    fn register( cx: Scope ) -> Element {
        cx.render( rsx!{
        head {
            title{
                "Web DIOXUS"
            }
            link {
                rel: "stylesheet",
                href: "https://monthebrice2000.github.io/dioxus_template_project/styles-e31f16350f020b31.css",
            }
        }
        body{
            div{
            class : "main",
            div {
                class : "header",
                a{
                    "href" : "http://127.0.0.1:3001/",
                    class : "title",
                    "Conduit"
                }
                div {
                    class : "nav",
                    a{
                        "href" : "http://127.0.0.1:3001/",
                        class : "item",
                        "Home"
                    }
                    a{
                        "href" : "http://127.0.0.1:3001/signin",
                        class : "item",
                        "Sign in"
                    }
                    a{
                        "href" : "http://127.0.0.1:3001/register",
                        class : "item active",
                        "Sign up"
                    }
                }
            },
            div {
                class : "body",
                h2{
                    class : "item title",
                    "Sign up"
                }
                a{
                    class : "item login",
                    href : "#",
                    "Have an account?"
                }
                form {
                    class : "item form",
                    method : "post",
                    action : "#",
                    name : "form",
                    input{
                        "type" : "text",
                        class : "input username",
                        name : "username",
                        placeholder : "Username"
                    }
                    input{
                        "type" : "email",
                        class : "input email",
                        name : "email",
                        placeholder : "Email"
                    }
                    input{
                        "type" : "submit",
                        class : "input submit",
                        name : "submit",
                        value : "Sign up"
                    }
                }
            }
            a {
                class : "footer",
                "href" : "https://github.com/gothinkster/angularjs-realworld-example-app",
                target : "_blank",
                span {
                    class : "logo",
                    i{
                        class : "fab fa-github",
                    }
                }
                span{
                    class : "title",
                    "Fork On Github"
                }

            }
        }
        }
    } )
    }
    let mut app = VirtualDom::new(register );
    let _ = app.rebuild();
    Html( dioxus::ssr::render_vdom( &app ))
}


async fn home() -> Html<String>{
    fn home( cx: Scope ) -> Element {
        cx.render( rsx!{
        head {
            title{
                "Web DIOXUS"
            }
            link {
                rel: "stylesheet",
                href: "https://monthebrice2000.github.io/dioxus_template_project/styles-e31f16350f020b31.css",
            }
        }
        body{
            div{
            class : "main home",
            div {
                class : "header",
                a{
                    "href" : "http://127.0.0.1:3001/",
                    class : "title",
                    "Conduit"
                }
                div {
                    class : "nav",
                    a{
                        "href" : "http://127.0.0.1:3001/",
                        class : "item active",
                        "Home"
                    }
                    a{
                        "href" : "http://127.0.0.1:3001/signin",
                        class : "item",
                        "Sign in"
                    }
                    a{
                        "href" : "http://127.0.0.1:3001/register",
                        class : "item",
                        "Sign up"
                    }
                }
            },
            div {
                class : "header_description",
                div {
                    class : "title",
                    "conduit"
                }
                div{
                    class : "description",
                    "A place to share your knowledge"
                }
            }
            div {
                class : "body",
                div {
                    class : "feeds",
                    div{
                        class : "nav",
                        div {
                            class : "item first",
                            "YOUR FEEDS"
                        }
                        div {
                            class : "item second active",
                            "GLOBAL FEEDS"
                        }
                        div {
                            class : "item third",
                            "ANOTHER SEARCH"
                        }
                    }
                    div {
                        class : "content second",
                        div {
                            class : "item_ first",
                            "ALL YOUR FEEDS"
                        }
                        div {
                            class : "item_ second",
                            "ALL GLOBAL FEEDS"
                        }
                        div {
                            class : "item_ third",
                            "ALL ANOTHER SEARCH"
                        }
                    }
                }
                div {
                    class : "aside",
                    div{
                        class : "title",
                        "Popular Tags"
                    }
                    a{
                        class : "title",
                        "href" : "#",
                        class : "tag",
                        "welcome"
                    }
                    a{
                        class : "title",
                        "href" : "#",
                        class : "tag",
                        "implementations"
                    }
                    a{
                        class : "title",
                        "href" : "#",
                        class : "tag",
                        "codebaseShow"
                    }
                    a{
                        class : "title",
                        "href" : "#",
                        class : "tag",
                        "introduction"
                    }
                }
            }
            a {
                class : "footer",
                "href" : "https://github.com/gothinkster/angularjs-realworld-example-app",
                target : "_blank",
                span {
                    class : "logo",
                    i{
                        class : "fab fa-github",
                    }
                }
                span{
                    class : "title",
                    "Fork On Github"
                }

            }
        }
      }
    } )
    }
    let mut app = VirtualDom::new(home);
    let _ = app.rebuild();
    Html( dioxus::ssr::render_vdom( &app ))
}


async fn login() -> Html<String> {
    fn login( cx: Scope ) -> Element {
        cx.render( rsx!{
        head {
            title{
                "Web DIOXUS"
            }
            link {
                rel: "stylesheet",
                href: "https://monthebrice2000.github.io/dioxus_template_project/styles-e31f16350f020b31.css",
            }
        }
        body{
            div{
            class : "main",
            div {
                class : "header",
                a{
                    "href" : "http://127.0.0.1:3001/",
                    class : "title",
                    "Conduit"
                }
                div {
                    class : "nav",
                    a{
                        "href" : "http://127.0.0.1:3001/",
                        class : "item",
                        "Home"
                    }
                    a{
                        "href" : "http://127.0.0.1:3001/signin",
                        class : "item active",
                        "Sign in"
                    }
                    a{
                        "href" : "http://127.0.0.1:3001/register",
                        class : "item",
                        "Sign up"
                    }
                }
            },
            div {
                class : "body",
                h2{
                    class : "item title",
                    "Sign in"
                }
                a{
                    class : "item register",
                    href : "#",
                    "Need An Account?"
                }
                form {
                    class : "item form",
                    method : "post",
                    action : "#",
                    name : "form",
                    input{
                        "type" : "email",
                        class : "input email",
                        name : "email",
                        placeholder : "Email"
                    }
                    input{
                        "type" : "password",
                        class : "input password",
                        name : "password",
                        placeholder : "Password"
                    }
                    input{
                        "type" : "submit",
                        class : "input submit",
                        name : "submit",
                        value : "Sign up"
                    }
                }
            }
            a {
                class : "footer",
                "href" : "https://github.com/gothinkster/angularjs-realworld-example-app",
                target : "_blank",
                span {
                    class : "logo",
                    img{
                        src : "https://monthebrice2000.github.io/dioxus_template_project/img/github.png",
                    }
                }
                span{
                    class : "title",
                    "Fork On Github"
                }

            }
        }
        }
    } )
    }
    let mut app = VirtualDom::new(login);
    let _ = app.rebuild();
    Html( dioxus::ssr::render_vdom( &app ))
}