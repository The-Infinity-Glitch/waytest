use crate::state;

use std::time::Duration;

use smithay::{
    backend::{
        allocator::dmabuf::Dmabuf,
        egl::EGLDevice,
        renderer::{
            damage::OutputDamageTracker, element::AsRenderElements, glow::GlowRenderer, ImportDma,
            ImportEgl,
        },
        winit::{self, WinitEvent, WinitEventLoop, WinitGraphicsBackend},
    },
    delegate_dmabuf,
    desktop::{layer_map_for_output, space::SpaceElement, LayerSurface},
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::{
        calloop::{
            timer::{TimeoutAction, Timer},
            EventLoop,
        },
        wayland_server::{Display, DisplayHandle},
        winit::platform::pump_events::PumpStatus,
    },
    utils::{Rectangle, Scale, Transform},
    wayland::{
        dmabuf::{
            DmabufFeedback, DmabufFeedbackBuilder, DmabufGlobal, DmabufHandler, DmabufState,
            ImportNotifier,
        },
        shell::wlr_layer::Layer,
    },
};

pub struct WinitData {
    backend: WinitGraphicsBackend<GlowRenderer>,
    damage_tracker: OutputDamageTracker,
    dmabuf_state: (DmabufState, DmabufGlobal, Option<DmabufFeedback>),
}

pub fn run_winit() {}
