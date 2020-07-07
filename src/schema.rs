use juniper::{EmptyMutation, GraphQLObject, RootNode};
use serde::Deserialize;

use crate::data::get_menu;

#[derive(GraphQLObject, Deserialize, Clone)]
pub struct Menu {
    pub name: MenuName,
    pub price: i32,
    pub menu_type: String,
}

#[derive(GraphQLObject, Deserialize, Clone)]
pub struct MenuName {
    pub th: String,
    pub en: String,
    pub jp: String,
}

pub struct QueryRoot {}

#[juniper::object]
impl QueryRoot {
    fn get_all_menu() -> Vec<Menu> {
        get_menu()
    }

    fn get_menu_by(name: String) -> Vec<Menu> {
        let search_key = name.to_lowercase();

        get_menu()
            .into_iter()
            .filter(move |menu| {
                menu.name.th.to_lowercase().contains(&search_key)
                    || menu.name.en.to_lowercase().contains(&search_key)
                    || menu.name.jp.to_lowercase().contains(&search_key)
            })
            .collect()
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
