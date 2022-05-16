use std::{rc::Rc, cell::RefCell, time::Duration};

use log::{error, info};
use noise::NoiseFn;
use fluvio_wasm_timer::Delay;
use yew_agent::{Agent, Job, AgentLink, HandlerId};
use futures::FutureExt;
const NOISE_OFFSET:(f64, f64) = (80.27, 23.4);

pub enum GameOfLifeInput{
    SetFunction(Box<dyn Fn(f64, f64)->f64>),
    SetKernel(Vec<((i64, i64), f64)>),
    SetTimeScale(f64),
    TogglePlay,
    InitVal(usize, usize, f64),
    InitNoise(usize, usize, f64, Box<dyn NoiseFn<[f64;2]>>),
    Step
}

pub enum GameOfLifeMessage{
    Step
}

pub struct GameOfLife{
    // Agent stuff
    link:AgentLink<Self>,
    client: Option<HandlerId>,

    // Simulation Parameters 
    step_function: Option<Box<dyn Fn(f64, f64)->f64>>,
    kernel: Option<Vec<((i64, i64), f64)>>,
    time_scale: f64,
    
    //play
    playing:bool,

    // State 
    initialized:bool,
    cells: [Rc<RefCell<Vec<Vec<f64>>>>; 2],
    odd : bool
}

impl GameOfLife{
    fn val_init(&mut self, (width, height):(usize, usize), val:f64){
        self.cells[0] = Rc::new(RefCell::new(vec![vec![val; width]; height]));
        self.cells[1] = Rc::new(RefCell::new(vec![vec![val; width]; height]));
        self.odd = false;
        self.initialized = true;
    }


    fn noise_init(&mut self, (width, height):(usize, usize), scale:f64, noise_fn:Box<dyn NoiseFn<[f64;2]>>){
        let mut cells = vec![];

        for y in 0..height {
            let mut line = vec![];
            for x in 0..width {
                line.push((noise_fn.get([(x as f64+NOISE_OFFSET.0)*scale, (y as f64+NOISE_OFFSET.1)*scale])+1.0)/2.0);
            }
            cells.push(line);
        }

        self.cells[0] = Rc::new(RefCell::new(cells));
        self.cells[1] = Rc::new(RefCell::new(vec![vec![0.0; width]; height]));
        self.odd = false;
        self.initialized = true;

    }

    fn step(&mut self){
        let start = js_sys::Date::now();
        let (step_function, kernel) = match (self.step_function.as_ref(), self.kernel.as_ref(), self.initialized){
            (Some(f), Some(k),true) => (f,k),
            _ => {
                error!("The simulation parameters have to be initialized to step");
                return;
            }
        };

        let src = self.cells[self.odd as usize].borrow();
        let mut dst = self.cells[!self.odd as usize].borrow_mut();
        
        let height = src.len();
        let width = src.first().unwrap().len();

        for (y, line) in src.iter().enumerate() {
            for (x, val) in line.iter().enumerate() {
                let x = x as i64;
                let y = y as i64;
                let sum = kernel.iter()
                    .map(|((kx,ky), val)| ((kx+x,ky+y),val))
                    .filter(|((x,y), _)|*x>=0 && *y>=0 && *x<width as i64 && *y<height as i64)
                    .map(|((x,y), val)| ((x as usize,y as usize), val))
                    .fold(0.0, |acc,((x,y), val)|acc+(*val * src[y][x])); 
                dst[y as usize][x as usize] = *val + self.time_scale*step_function.as_ref()(*val, sum);
            }
        }
        //info!("step_duration : {:?}", (js_sys::Date::now() - start));
        self.odd = !self.odd;
    }

    
}

impl Agent for GameOfLife{
    type Reach = Job<Self>;

    type Message = GameOfLifeMessage;

    type Input = GameOfLifeInput;

    type Output = Rc<RefCell<Vec<Vec<f64>>>>;

    fn create(link: yew_agent::AgentLink<Self>) -> Self {        
        Self{
            link,
            client:None, 
            step_function: None,
            kernel: None,
            time_scale: 1.0,
            cells: [
                Rc::new(RefCell::new(vec![])),
                Rc::new(RefCell::new(vec![])),
                
            ],
            odd: false,
            initialized: false,
            playing: false,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            GameOfLifeMessage::Step => {
                self.step();
                if self.playing{
                    let f = Delay::new(Duration::from_millis(10)).map(|_|GameOfLifeMessage::Step);
                    self.link.send_future(f);
                }
            },
        }
        if let Some(c) = self.client {
            self.link.respond(c, self.cells[self.odd as usize].clone())
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: yew_agent::HandlerId) {
        let notify = match msg {
            GameOfLifeInput::SetFunction(f) => {
                self.step_function = Some(f);
                false
            },
            GameOfLifeInput::SetKernel(k) => {
                self.kernel=Some(k);
                false
            },
            GameOfLifeInput::SetTimeScale(t) => {
                self.time_scale = t;
                false
            },
            GameOfLifeInput::InitVal(w, h, v) => {
                self.val_init((w,h), v);
                true
            }
            GameOfLifeInput::InitNoise(w, h, s, n) => {
                self.noise_init((w,h), s, n);
                true
            },
            GameOfLifeInput::Step => {
                self.step();
                true
            },
            GameOfLifeInput::TogglePlay => {
                self.playing = !self.playing;
                if self.playing{
                    let f = Delay::new(Duration::from_millis(10)).map(|_|GameOfLifeMessage::Step);
                    self.link.send_future(f);
                }
                false
            },
        };
        info!("{}",self.odd);
        if let (true, Some(c)) = (notify,self.client) {
            self.link.respond(c, self.cells[self.odd as usize].clone())
        }
    }

    fn connected(&mut self, id: yew_agent::HandlerId) {
        self.client = Some(id);
    }

    fn disconnected(&mut self, _id: yew_agent::HandlerId) {
        self.client = None;
    }

    fn destroy(&mut self) {}

    fn name_of_resource() -> &'static str {
        "main.js"
    }

    fn resource_path_is_relative() -> bool {
        false
    }

    fn is_module() -> bool {
        false
    }
    
}