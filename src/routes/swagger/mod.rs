use std::fs::File;

use rocket::Rocket;
// use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

pub trait SwaggerRouteInitialized {
    fn mount_swagger_route(self) -> Self;
}

impl SwaggerRouteInitialized for Rocket {
    fn mount_swagger_route(self) -> Self {
        self.mount("/api/v1/swagger", routes![open_api])
        // .mount("/api/v1/swagger", make_swagger_ui(&get_docs()))
        // todo fix swagger ui
    }
}

#[get("/openapi.json")]
pub fn open_api() -> File {
    File::open("./openapi.json").expect("file not found")
}

// todo fix swagger ui
// fn get_docs() -> SwaggerUIConfig {
//     SwaggerUIConfig {
//         url: "/api/v1/swagger/openapi.json".to_string(),
//         ..Default::default()
//     }
// }
