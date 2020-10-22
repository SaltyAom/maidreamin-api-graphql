use std::collections::HashMap;
use std::sync::Mutex;

use juniper::{Context, EmptyMutation, EmptySubscription, GraphQLObject, RootNode};
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

pub struct Cache {
    pub cache: Mutex<HashMap<String, Vec<Menu>>>,
}

impl Context for Cache {}

pub struct QueryRoot {}

#[juniper::graphql_object(
    Context = Cache
)]
impl QueryRoot {
    #[inline(always)]
    fn get_all_menu() -> Vec<Menu> {
        get_menu()
    }

    #[inline(always)]
    fn get_menu_by(context: &mut Cache, name: String) -> Vec<Menu> {
        let search_key = name.to_lowercase();
        let mut cache = context.cache.lock().unwrap();

        if cache.contains_key(&search_key) {
            return cache.get(&search_key).unwrap().to_vec();
        }

        let filtered_menu: Vec<Menu> = get_menu()
            .iter()
            .filter(move |&menu| {
                menu.name.th.to_lowercase().contains(&search_key)
                    || menu.name.en.to_lowercase().contains(&search_key)
                    || menu.name.jp.to_lowercase().contains(&search_key)
            })
            .cloned()
            .collect();

        cache.insert(name.to_lowercase(), filtered_menu.to_vec());

        filtered_menu
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Cache>, EmptySubscription<Cache>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
