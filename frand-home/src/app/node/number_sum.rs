use yew::*;
use frand_node::*;

#[node]
#[derive(yew::Properties)]
pub struct NumberSum {
    pub a: i32,
    pub b: i32,
    pub sum: i32,
}

#[cfg(not(target_arch = "wasm32"))]
impl NumberSum {
    pub fn emit_expensive_sum(&self) {
        use tokio::time::sleep;
        use std::time::Duration;

        let (av, bv) = (*self.a, *self.b);
        self.sum.emit_future(async move {
            sleep(Duration::from_millis(200)).await;
            av + bv
        });
    }
}

#[function_component]
pub fn NumberSumView(node: &NumberSum) -> Html {
    log::debug!("NumberSum::view");
    let a = node.a.clone();
    let b = node.b.clone();
    let sum = node.sum.clone();

    html! {
        <span> { format!("{a} + {b} : {sum}") } </span>
    }
}