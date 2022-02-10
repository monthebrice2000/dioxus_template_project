extern crate dioxus;

use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!{
        div {
            "class" : "main_1",
            div{
                class: "header",
                div{
                    class: "header_top",
                    div{
                        class: "logo",
                        "Conduit"
                    }
                    div{
                        class: "nav",
                        span{
                            class: "link",
                            "Home"
                        }
                        span{
                            class: "link",
                            "Sing In"
                        }
                        span{
                            class: "link",
                            "Sign Up"
                        }
                    }
                }
                div{
                    class: "header_bottom",
                    h2 {
                        "Conduit"
                    }
                    p{
                        "A Place To Share Your Kwonledge"
                    }
                }
            },
            div{
                class: "main_2",
                "MAIN"
            },
            div{
                "class" : "footer",
                span{
                    class : "logo",
                    img{
                        src: "img/github.png",
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
