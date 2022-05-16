use std::cell::RefCell;
use std::f64::consts::E;
use std::rc::Rc;

use crate::config_panel::{ConfigPanel, ConfigPanelEnum};
use crate::gameoflife::agent::{GameOfLife, GameOfLifeInput};
use crate::kernel_editor::KernelEditor;
use crate::gridcanvas::GridCanvas;
use crate::play_controls::PlayControls;

use log::info;
use noise::{Perlin, NoiseFn};
use yew::prelude::*;
use yew_agent::use_bridge;

const BASIC_KERNEL:[((i64,i64),f64);8] = [
    ((-1,-1),1.0),
    ((-1,0),1.0),
    ((-1,1),1.0),
    ((0,-1),1.0),
    ((0,1),1.0),
    ((1,-1),1.0),
    ((1,0),1.0),
    ((1,1),1.0),
];

const WIDE_KERNEL:[((i64,i64),f64);20] = [
    ((-1,-1),1.0),
    ((-1,0),1.0),
    ((-1,1),1.0),
    ((0,-1),1.0),
    ((0,1),1.0),
    ((1,-1),1.0),
    ((1,0),1.0),
    ((1,1),1.0),
    ((-2,-1),0.25),
    ((-2,0),0.5),
    ((-2,1),0.25),
    ((0,-2),0.5),
    ((-1,-2),0.25),
    ((1,-2),0.25),
    ((0,2),0.5),
    ((1,2),0.25),
    ((-1,2),0.25),
    ((2,-1),0.25),
    ((2,0),0.5),
    ((2,1),0.25),
];

const OPTIMAL:f64 = 7.0;
const LIVABLE_SPREAD:f64 = 3.5;
const ALIVE_SHARPNESS:f64 = 10.0;
const ALIVE_OFFSET:f64 = 0.2;
const BACKGROUD_GENERATION:f64 = 0.2;

// ALIVE_SHARPNESS*sigmoid(-alive) - 1.0 + BACKGROUD_GENERATION + 
fn basic_evolution_function(alive:f64, neigbour:f64)->f64{
    let liveliness_contrib = 1.0/(1.0 + E.powf((alive-ALIVE_OFFSET)*ALIVE_SHARPNESS))-1.0+BACKGROUD_GENERATION;
    let crouding_contrib = E.powf(-(neigbour-OPTIMAL).powi(2)/LIVABLE_SPREAD.powi(2)) ;
    crouding_contrib + liveliness_contrib
}

#[function_component(App)]
pub fn app()->Html{
    
    // States
    let size = use_state(||(100,100));
    let grid_ref = use_state::<Option<Rc<RefCell<Vec<Vec<f64>>>>>,_>(||None);
    
    // Agents
    let gol = {
        let grid_ref = grid_ref.clone();
        use_bridge::<GameOfLife,_>(move |output|{
            grid_ref.set(Some(output))
        })
    };

    // On Start
    {
        let gol = gol.clone();
        use_effect_with_deps(move|_|{
            info!("called");
            gol.send(GameOfLifeInput::SetKernel(Vec::from(WIDE_KERNEL)));
            gol.send(GameOfLifeInput::SetTimeScale(0.1));
            gol.send(GameOfLifeInput::SetFunction(Box::new(basic_evolution_function)));
            ||{}
        },());
    }

    let init_callback = {
        let size = size.clone();
        let gol = gol.clone();
        Callback::from(move |conf|{
            match conf {
                ConfigPanelEnum::Init(w, h) => {
                    gol.send(GameOfLifeInput::InitVal(w, h, 0.5));
                    size.set((w,h))
                },
                ConfigPanelEnum::InitWPerlin(w, h, s) => {
                    let n = Perlin::new();
                    let v = n.get([42.4, 37.7]);
                    info!("{v}");
                    gol.send(GameOfLifeInput::InitNoise(w, h, s, Box::new(n)));
                    size.set((w,h))

                },
            }
        })
    };

    let toggle_cb = {
        let gol = gol.clone();
        Callback::from(move|_|{
            gol.send(GameOfLifeInput::TogglePlay);
        })
    };
    let step_cb = {
        let gol = gol.clone();
        Callback::from(move|_|{
            gol.send(GameOfLifeInput::Step);
        })
    };

    let dimentions = *size;
    html!{ 
        <>
            <nav >
                <h1>{"Modular Continuous Conaway's Game Of Life"}</h1>
                <a href="">
                    <i class="fab fa-github"></i>
                </a>
            </nav>
            <article class="hero">
                <div class="container">
                if let Some(grid) = &*grid_ref{
                    <GridCanvas {dimentions} values={grid} />
                    <PlayControls play_toggle_callback={toggle_cb} step_callback={step_cb}/>
                }
                </div>
            </article>
            <div class="container">
                <h3>{"Config"}</h3>
                <ConfigPanel {init_callback}/>
                <h3>{"Kernel"}</h3>
                <KernelEditor/>
                <h3>{"Function"}</h3>
                {"TBA"}
            </div>
        </>
    }
}