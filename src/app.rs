use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    yew_html_ext::html! {
        <>
        <h1>{ "Help!" }</h1>
        <h2>{ "What even!" }</h2>
        </>
    }
}

// this worked in 438db0e8547eae22f79a18f9851399c34e3ea8f6

// #[function_component]
// pub fn PossibleRegressionA() -> Html {
//     let foo = Some(1);
//
//     yew_html_ext::html! {
//         match foo {
//             Some(_) => "habituated",
//             None => "unhabituated",
//         }
//     }
// }

// this worked in 438db0e8547eae22f79a18f9851399c34e3ea8f6
//
//
// #[function_component]
// pub fn PossibleRegressionB() -> Html {
//     let foo = Some(1);
//
//     let bar = html! {};
//
//     yew_html_ext::html! {
//         match foo {
//             Some(_) => bar,
//             None => bar,
//         }
//     }
// }
