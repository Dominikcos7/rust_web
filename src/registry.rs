use crate::controllers::{
    index_controller::IndexController,
};
use crate::Response;
use std::collections::HashMap;

pub struct Registry {
    route_map: HashMap<&'static str, fn() -> Response>
}

impl Registry {
    pub fn get(self: &Self, path: &str) -> Option<fn() -> Response> {
        self.route_map.get(path).copied()
    }

    pub fn init(self: &mut Self) {
        self.route_map.insert("/", IndexController::action_index);
    }

    pub fn new() -> Self {
        Registry {
            route_map: HashMap::new()
        }
    }
}