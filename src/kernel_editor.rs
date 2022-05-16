use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

fn draw_row(_index:usize, row:&Vec<f64>)->Html{
    let row = row.iter()
        .map(|val|format!("{val}"))
        .map(|value|{
            html!{
                <td><input type="number" step="0.1" value={value}/></td>
            }
        });

    html!{
        <tr>
            {for row}
        </tr>
    }
}

#[function_component(KernelEditor)]
pub fn kernel_editor()->Html{
    let kernel: UseStateHandle<Vec<Vec<f64>>> = use_state(||vec![]);
    
    let rows = kernel.iter()
        .enumerate()
        .map(|(i, row)|draw_row(i, row));

    
    
    let handle_size = {
        let kernel = kernel.clone();
        Callback::from(move |ev:Event|{
            if let Some(target) = ev.target().and_then(|tg|tg.dyn_into::<HtmlInputElement>().ok()) {
                let size = target.value_as_number() as usize;
                kernel.set(vec![vec![1.0; size]; size])
            }
        })
    };

    html!{
        <>
        <label for="width">
        {"Size"}
        <input onchange={handle_size} type="number" id="width" min="3" step="2" max="7" name="width" placeholder="Width"/>
        </label>
        <table>
            {for rows}
        </table>
        <button>{"save"}</button>
        </>
    }
}