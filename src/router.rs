use std::collections::HashMap;

use crate::{
    controller:: {
        Controller,
        dummy_controller::DummyController,
    }, request::Request, response::Response
};

pub struct Router<'a> {
    controller_map: HashMap<&'a str, Box<dyn Controller>>
}

impl Router<'_> {
    pub fn handle_request(request: Request) -> Response {
        let (controller, action) = Self::parse_path(request.get_path());
        DummyController::respond_404_not_found()
    }

    fn parse_path(path: &String) -> (&str, &str) {
        let parts: Vec<&str> = path.trim_matches('/').split("/").collect();
        dbg!(&parts);
        dbg!(parts.len());

        let controller = parts.get(0).copied().unwrap_or("index");
        let controller = if controller.is_empty() {"index"} else {controller};
        let action = parts.get(1).copied().unwrap_or("index");

        dbg!(&controller);
        dbg!(&action);

        (controller, action)
    }

    fn read_map_config() {

    }
}

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use super::*;

    #[test]
    fn should_parse_empty_path() {
        let controller_name = "index";
        let action_name = "index";

        let path = "/".to_string();
        let (controller, action) = Router::parse_path(&path);

        assert_eq!(controller, controller_name);
        assert_eq!(action, action_name);
    }

    #[test]
    fn should_parse_path_with_controller_and_without_action() {
        let controller_name = "some-controller";
        let action_name = "index";

        let path = format!("/{}", controller_name);
        let (controller, action) = Router::parse_path(&path);

        assert_eq!(controller, controller_name);
        assert_eq!(action, action_name);
    }

    #[test]
    fn should_parse_path_with_controller_and_action_name() {
        let controller_name = "some-controller";
        let action_name = "some-action";

        let path = format!("/{}/{}", controller_name, action_name);
        let (controller, action) = Router::parse_path(&path);

        assert_eq!(controller, controller_name);
        assert_eq!(action, action_name);
    }
}