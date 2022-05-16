use std::{rc::Rc, cell::RefCell};

use log::warn;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use yew::{prelude::*};


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct GridCanvasProps{
    pub dimentions:(usize,usize),
    pub values:Rc<RefCell<Vec<Vec<f64>>>>
}

pub enum GridCanvasMessage{

}

pub struct GridCanvas{
    canvas_ref: NodeRef,
    canvas: Option<HtmlCanvasElement>,
    rendering_context: Option<CanvasRenderingContext2d>
}

impl GridCanvas {
    fn draw_cells(&self, ctx:&Context<Self>){
        let (_, render_ctx) = match (self.canvas.as_ref(), self.rendering_context.as_ref()) {
            (Some(cnva), Some(ctx)) => (cnva,ctx),
            _ => {
                warn!("The canvas seems to be uninitiated");
                return;
            }
        };

        let grid = ctx.props().values.clone();
        let grid = grid.borrow();
        let (width, height) = ctx.props().dimentions;
        let grid_width = grid.first().map(|l|l.len()).unwrap_or(0);
        let grid_height = grid.len();
        let x_ratio = width as f64/grid_width as f64;
        let y_ratio = height as f64/grid_height as f64;
        for (y, line) in grid.iter().enumerate() {
            for (x, val) in line.iter().enumerate() {
                let val = (val*255.0) as u8;
                let clr = rgb::RGB::new(val, val, val);
                render_ctx.set_fill_style(&format!("#{}", hex::encode(clr)).into());
                render_ctx.fill_rect(x as f64*x_ratio, y as f64*y_ratio, x_ratio, y_ratio);
            }
        }
    }
    
}

impl Component for GridCanvas{
    type Message = GridCanvasMessage;

    type Properties = GridCanvasProps;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self{
            canvas_ref: NodeRef::default(),
            canvas: None,
            rendering_context: None,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let (width, height) = ctx.props().dimentions;
        html!{
            <canvas id="grid-canvas" ref={self.canvas_ref.clone()} width={width.to_string()} height={height.to_string()}/>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.draw_cells(ctx);
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        self.canvas = self.canvas_ref.cast::<HtmlCanvasElement>();
        if self.canvas.is_none() {
            warn!("Couldn't cast the canvas reference into the HTML Canvas Element\n\tThe GridCanvas component wont work");
            return;
        }
        let canvas = self.canvas.as_ref().unwrap();
        let rendering_context = match canvas.get_context("2d") {
            Ok(Some(ctx)) => ctx,
            Ok(None) => {
                warn!("Couldn't get the rendering context from the canvas for an unknown cause\n\tThe GridCanvas component wont work");
                return;
            },
            Err(err) => {
                warn!("Couldn't get the rendering context from the canvas\n\tThe GridCanvas component wont work\nUnderlying cause : {err:?}");
                return;
            },
        };
        self.rendering_context = match rendering_context.dyn_into::<CanvasRenderingContext2d>() {
            Ok(ctx) => Some(ctx),
            Err(err) => {
                warn!("Couldn't cast the rendering context\n\tThe GridCanvas component wont work\nUnderlying cause : {err:?}");
                None
            }
        };

        self.draw_cells(ctx);
        
    }
}