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
                href: "https://monthebrice2000.github.io/dioxus_web_template/css/style.css",
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
                    svg{
                        xmlns : "http://www.w3.org/2000/svg",
                        "viewBox" : "0 0 496 512",
                        fill : "white",
                        path {
                            d : "M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"
                        }
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
                href: "https://monthebrice2000.github.io/dioxus_web_template/css/style.css",
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
                    svg{
                        xmlns : "http://www.w3.org/2000/svg",
                        "viewBox" : "0 0 496 512",
                        fill : "white",
                        path {
                            d : "M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"
                        }
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
                href: "https://monthebrice2000.github.io/dioxus_web_template/css/style.css",
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
                    svg{
                        xmlns : "http://www.w3.org/2000/svg",
                        "viewBox" : "0 0 496 512",
                        fill : "white",
                        path {
                            d : "M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"
                        }
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