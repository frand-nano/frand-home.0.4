use yew::Renderer;
use super::simple_component::SimpleComponent;

#[allow(unused)]
pub fn render() {
    Renderer::<SimpleComponent>::new().render();    
}