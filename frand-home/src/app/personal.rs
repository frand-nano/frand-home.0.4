use yew::*;
use frand_node::*;
use crate::app::view::IncButton;

#[node_macro]
#[derive(Properties)]
pub struct Personal {
    pub number1: i32,
    pub number2: i32,
    pub number3: i32,
    pub number4: i32,
}

personal_macro!{}

impl Component for Personal {
    type Message = ();
    type Properties = Self;

    fn create(context: &Context<Self>) -> Self {
        log::debug!("Personal::create");
        context.props().clone()
    }

    fn view(&self, _: &Context<Self>) -> Html {     
        log::debug!("Personal::view");   
        html! {
            <div>
                <IncButton ..self.number1.clone().into() />
                <IncButton ..self.number2.clone().into() />
                <IncButton ..self.number3.clone().into() />
                <IncButton ..self.number4.clone().into() />
            </div>
        }
    }
}
