use crate::schema::Menu;

lazy_static! {
    static ref DREAMIN: Vec<Menu> =
        serde_json::from_str(include_str!("../static/dreamin.json")).unwrap();
}

#[inline(always)]
pub fn get_menu() -> Vec<Menu> {
    DREAMIN.to_owned()
}
