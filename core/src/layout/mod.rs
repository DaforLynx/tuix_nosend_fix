

pub(crate) mod cache;

pub(crate) mod node;

pub(crate) mod hierarchy;

use morphorm::{Cache, Hierarchy};
use crate::{Event, GeometryChanged, Propagation, State, Tree, WindowEvent};

pub(crate) fn geometry_changed(state: &mut State, tree: &Tree) {
    for node in tree.down_iter() {
        let geometry_changed = state.data.geometry_changed(node);

        state.insert_event(Event::new(WindowEvent::GeometryChanged(GeometryChanged {
            posx: geometry_changed.contains(morphorm::GeometryChanged::POSX_CHANGED),
            posy: geometry_changed.contains(morphorm::GeometryChanged::POSY_CHANGED),
            width: geometry_changed.contains(morphorm::GeometryChanged::WIDTH_CHANGED),
            height: geometry_changed.contains(morphorm::GeometryChanged::HEIGHT_CHANGED),
        })).target(node).propagate(Propagation::Down));

        state.data.set_geo_changed(node, morphorm::GeometryChanged::POSX_CHANGED, false);
        state.data.set_geo_changed(node, morphorm::GeometryChanged::POSY_CHANGED, false);
        state.data.set_geo_changed(node, morphorm::GeometryChanged::WIDTH_CHANGED, false);
        state.data.set_geo_changed(node, morphorm::GeometryChanged::HEIGHT_CHANGED, false);
    }
} 
