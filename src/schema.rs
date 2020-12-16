use std::sync::RwLock;

use hashbrown::HashMap;

use juniper::{graphql_object, Context, EmptyMutation, EmptySubscription, GraphQLObject, RootNode};

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
    pub cache: RwLock<HashMap<String, Vec<Menu>>>,
}

impl Context for Cache {}

pub struct Query;

#[graphql_object(
    Context = Cache
)]
impl Query {
    #[inline(always)]
    async fn get_all_menu() -> Vec<Menu> {
        get_menu()
    }

    #[inline(always)]
    async fn get_menu_by(context: &mut Cache, name: String) -> Vec<Menu> {
        let search_key = name.to_lowercase();

        {
            let readable_cache = context.cache.read().unwrap();

            if readable_cache.contains_key(&search_key) {
                return readable_cache.get(&search_key).unwrap().to_owned();
            }
        }

        let filtered_menu: Vec<Menu> = get_menu()
            .into_iter()
            .filter(move |menu| {
                menu.name.th.to_lowercase().contains(&search_key)
                    || menu.name.en.to_lowercase().contains(&search_key)
                    || menu.name.jp.to_lowercase().contains(&search_key)
            })
            .collect();

        let mut writable_cache = context.cache.write().unwrap();

        writable_cache.insert(name.to_lowercase(), filtered_menu.to_owned());

        filtered_menu
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Cache>, EmptySubscription<Cache>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}
