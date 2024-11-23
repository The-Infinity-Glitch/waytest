mod compositor;
mod xdg_shell;

use crate::state;
use smithay::{
    delegate_data_device, delegate_output, delegate_seat,
    input::{Seat, SeatHandler, SeatState},
    reexports::wayland_server::{protocol::wl_surface::WlSurface, Resource},
    wayland::{
        output::OutputHandler,
        selection::{
            data_device::{
                set_data_device_focus, ClientDndGrabHandler, DataDeviceHandler,
                ServerDndGrabHandler,
            },
            SelectionHandler,
        },
    },
};

impl ServerDndGrabHandler for state::Waytest {}

impl ClientDndGrabHandler for state::Waytest {}

impl SelectionHandler for state::Waytest {
    type SelectionUserData = ();

    fn new_selection(
        &mut self,
        ty: smithay::wayland::selection::SelectionTarget,
        source: Option<smithay::wayland::selection::SelectionSource>,
        seat: Seat<Self>,
    ) {
    }

    fn send_selection(
        &mut self,
        ty: smithay::wayland::selection::SelectionTarget,
        mime_type: String,
        fd: std::os::unix::prelude::OwnedFd,
        seat: Seat<Self>,
        user_data: &Self::SelectionUserData,
    ) {
    }
}

impl DataDeviceHandler for state::Waytest {
    fn data_device_state(&self) -> &smithay::wayland::selection::data_device::DataDeviceState {
        &self.data_device_state
    }

    fn action_choice(
        &mut self,
        available: smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction,
        preferred: smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction,
    ) -> smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction {
        smithay::wayland::selection::data_device::default_action_chooser(available, preferred)
    }
}

delegate_data_device!(state::Waytest);

impl SeatHandler for state::Waytest {
    type KeyboardFocus = WlSurface;

    type PointerFocus = WlSurface;

    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn focus_changed(&mut self, _seat: &Seat<Self>, _focused: Option<&Self::KeyboardFocus>) {
        let display_handle = &self.display_handle;
        let client = _focused.and_then(|s| display_handle.get_client(s.id()).ok());

        set_data_device_focus(display_handle, _seat, client);
    }

    fn cursor_image(
        &mut self,
        _seat: &Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }
}

delegate_seat!(state::Waytest);

impl OutputHandler for state::Waytest {}

delegate_output!(state::Waytest);
