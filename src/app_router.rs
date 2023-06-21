use yew::prelude::*;
use yew_router::prelude::*;

pub struct AppRouter;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/env")]
    Environment,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Environment => html! { <Environment /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

impl Component for AppRouter {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

#[function_component(Environment)]
fn environment() -> Html {
    let navigator = use_navigator().unwrap();

    let env = option_env!("CARGO_PROFILE").unwrap_or("UNKNOWN");

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ env }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
