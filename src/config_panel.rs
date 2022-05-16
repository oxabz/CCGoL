use log::info;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug)]
pub enum ConfigPanelEnum{
    Init(usize, usize),
    InitWPerlin(usize, usize, f64),
}

#[derive(Properties, PartialEq)]
pub struct ConfigPanelProps{
    pub init_callback:Callback<ConfigPanelEnum>,
}


#[function_component(ConfigPanel)]
pub fn config_panel(props: &ConfigPanelProps)->Html{
    let width = use_state(||20);
    let height = use_state(||20);
    let scale = use_state(||0.2);
    
    let handle_width_change = {
        let width = width.clone();
        Callback::from(move|event:Event|{
            if let Some(target) = event.target().and_then(|tg|tg.dyn_into::<HtmlInputElement>().ok()) {
                let size = target.value_as_number() as usize;
                width.set(size);
            }
        })
    };

    let handle_height_change = {
        let height = height.clone();
        Callback::from(move|event:Event|{
            if let Some(target) = event.target().and_then(|tg|tg.dyn_into::<HtmlInputElement>().ok()) {
                let size = target.value_as_number() as usize;
                height.set(size);
            }
        })
    };

    let handle_scale_change = {
        let scale = scale.clone();
        Callback::from(move|event:Event|{
            if let Some(target) = event.target().and_then(|tg|tg.dyn_into::<HtmlInputElement>().ok()) {
                let s = target.value_as_number() as f64;
                scale.set(s);
            }
        })
    };


    {
        let width =  *width;
        let height =  *height;
        let scale =  *scale;
        let cb = props.init_callback.clone();
        let handle_init = 
            Callback::from(move|e:MouseEvent|{e.prevent_default(); cb.emit(ConfigPanelEnum::Init(width, height))});

        let cb = props.init_callback.clone();
        let handle_init_with_perlin = 
            Callback::from(move|e:MouseEvent|{e.prevent_default(); cb.emit(ConfigPanelEnum::InitWPerlin(width, height, scale))});
    
        let width = format!("{width}");
        let height = format!("{height}");
        let scale = format!("{scale}");
        html!{
            <form>
                <div class="grid">
                    <label for="width">
                    {"Width"}
                    <input onchange={handle_width_change} type="number" id="width" name="width" placeholder="Width" value={width}/>
                    </label>

                    <label for="height">
                    {"Height"}
                    <input onchange={handle_height_change} type="number" id="height" name="height" placeholder="Height" value={height}/>
                    </label>
                </div>
                <button onclick={handle_init}>{"Init"}</button>
                <div class="grid">
                    <label for="noise-scale">
                    {"Noise Scale"}
                    <input onchange={handle_scale_change} type="number" id="noise-scale" name="noise-scale" placeholder="Scale" value={scale}/>
                    </label>

                </div>
                <button onclick={handle_init_with_perlin}>{"Init With Perlin"}</button>
            </form>
        }
    }   
}