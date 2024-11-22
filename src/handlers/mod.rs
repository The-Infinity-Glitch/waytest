mod compositor;

use crate::state;
use smithay::{
    delegate_seat,
    input::{Seat, SeatHandler, SeatState},
    reexports::wayland_server::{protocol::wl_surface::WlSurface, Resource},
    wayland::selection::data_device::set_data_device_focus,
};

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
