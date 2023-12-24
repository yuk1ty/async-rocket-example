pub mod db;
pub mod errors;
pub mod models;
pub mod routing;

// uuid::Uuid itself doesn't implement `rocket::request::FromParam` and that causes that
// `uuid::Uuid` can't be used as a parameter in a routing. This is a workaround for that.
// `rocket_contrib::Uuid` would be helpful if they could catch up Rocket v0.5.x.
pub struct Uuid(uuid::Uuid);

impl<'r> rocket::request::FromParam<'r> for Uuid {
    type Error = uuid::Error;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        uuid::Uuid::parse_str(param).map(Uuid)
    }
}
