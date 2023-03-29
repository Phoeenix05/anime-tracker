use rocket::Route;

mod post;

pub fn routes() -> Vec<Route> {
    routes![post::manga, post::chapter]
}
