use yew::prelude::*;

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct ControlProps {
    pub play_toggle_callback:Callback<()>,
    pub step_callback:Callback<()>
}

#[function_component(PlayControls)]
pub fn play_controls(props:&ControlProps)->Html{
    let ControlProps{play_toggle_callback, step_callback} = props.clone();
    let handle_click_play = Callback::from(move |_|play_toggle_callback.emit(()));
    let handle_step = Callback::from(move |_|step_callback.emit(()));

    html!{
        <div id="play-controls" class="grid">
            <button onclick={handle_click_play}></button>
            <button onclick={handle_step}></button>
        </div>
    }
}