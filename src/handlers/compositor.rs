use smithay::wayland::compositor::CompositorHandler;

use crate::state;

impl CompositorHandler for state::Waytest {
    fn compositor_state(&mut self) -> &mut smithay::wayland::compositor::CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(
        &self,
        client: &'a smithay::reexports::wayland_server::Client,
    ) -> &'a smithay::wayland::compositor::CompositorClientState {
        &client
            .get_data::<state::ClientState>()
            .unwrap()
            .compositor_state
    }

    fn new_surface(
        &mut self,
        surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
        let _ = surface;
    }

    fn new_subsurface(
        &mut self,
        surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
        parent: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
        let _ = surface;
        let _ = parent;
    }

    fn commit(
        &mut self,
        surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
        todo!()
    }

    fn destroyed(
        &mut self,
        _surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
    }
}
