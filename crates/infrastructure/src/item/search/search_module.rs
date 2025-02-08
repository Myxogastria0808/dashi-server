use domain::{
    entity::data_type::{meilisearch::MeilisearchData, search_item::SearchItemData},
    value_object::error::item::search::SearchItemError,
};
use meilisearch_sdk::client::Client;

pub(super) async fn search(
    meilisearch: Client,
    keywords: String,
) -> Result<Vec<SearchItemData>, SearchItemError> {
    ////* validation *////
    if keywords.chars().count() == 0 {
        return Err(SearchItemError::EmptyKeywordsError);
    }

    ////* operation *////
    //* convert + to half space *//
    let keywords = keywords.replace("+", " ");
    //* get search result from MeiliSearch *//
    let meilisearch_items: Vec<MeilisearchData> = meilisearch
        .index("item")
        .search()
        .with_query(&keywords)
        .execute()
        .await?
        .hits
        .into_iter()
        .map(|item| item.result)
        .collect();
    //* convert MeilisearchData to SearchItemData *//
    let search_item_data: Vec<SearchItemData> = meilisearch_items
        .into_iter()
        .map(|item| SearchItemData {
            id: item.id,
            visible_id: item.visible_id,
            name: item.name,
            connector: item.connector,
            is_rent: item.is_rent,
            color: item.color,
        })
        .collect();

    Ok(search_item_data)
}
