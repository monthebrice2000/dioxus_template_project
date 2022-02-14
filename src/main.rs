extern crate dioxus;

use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render( rsx!{
        Home{
            score : 3,
        }
    })
}

#[derive(PartialEq, Props)]
struct RegisterProps {
    score: i32
}

fn Register(cx: Scope<RegisterProps>) -> Element {
    cx.render(rsx!{
        div{
            class : "main",
            div {
                class : "header",
                a{
                    "href" : "#",
                    class : "title",
                    "Conduit"
                }
                div {
                    class : "nav",
                    a{
                        "href" : "#",
                        class : "item active",
                        "Home"
                    }
                    a{
                        "href" : "#",
                        class : "item",
                        "Sign in"
                    }
                    a{
                        "href" : "#",
                        class : "item",
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
    })
}

#[derive(PartialEq, Props)]
struct HomeProps {
    score: i32
}

fn Home(cx: Scope<HomeProps>) -> Element {
    cx.render(rsx!{
        div{
            class : "main home",
            div {
                class : "header",
                a{
                    "href" : "#",
                    class : "title",
                    "Conduit"
                }
                div {
                    class : "nav",
                    a{
                        "href" : "#",
                        class : "item active",
                        "Home"
                    }
                    a{
                        "href" : "#",
                        class : "item",
                        "Sign in"
                    }
                    a{
                        "href" : "#",
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
    })
}

#[derive(PartialEq, Props)]
struct LoginProps {
    score: i32
}

fn Login(cx: Scope<LoginProps>) -> Element {
    cx.render(rsx!{
        div{
            class : "main",
            div {
                class : "header",
                a{
                    "href" : "#",
                    class : "title",
                    "Conduit"
                }
                div {
                    class : "nav",
                    a{
                        "href" : "#",
                        class : "item",
                        "Home"
                    }
                    a{
                        "href" : "#",
                        class : "item active",
                        "Sign in"
                    }
                    a{
                        "href" : "#",
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
    })
}