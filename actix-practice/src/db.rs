use crate::{constants, dto::BookRequest, error::Error::*, models::Book, Result};
// use bson::DateTime
use chrono::prelude::*;

use futures::StreamExt;
use mongodb::bson::{doc, document::Document, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
    pub database: String,
    pub collection: String,
}

impl DB {
    pub async fn init(uri: String, db: String, collection: String) -> Result<Self> {
        let mut client_options = ClientOptions::parse(uri).await?;
        client_options.app_name = Some("store_books".to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
            database: db,
            collection: collection,
        })
    }

    pub async fn fetch_books(&self) -> Result<Vec<Book>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Book> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_book(&doc?)?);
        }
        Ok(result)
    }

    pub async fn create_book(&self, entry: &BookRequest) -> Result<()> {
        let doc = doc! {
            constants::NAME: entry.name.clone(),
            constants::AUTHOR: entry.author.clone(),
            constants::NUM_PAGES: entry.num_pages as i32,
            constants::ADDED_AT: Utc::now(),
            constants::TAGS: entry.tags.clone(),
        };

        self.get_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    fn get_collection<T>(&self) -> Collection<T> {
        dbg!("get_collection");
        self.client
            .database(&self.database)
            .collection(&self.collection)
    }

    fn doc_to_book(&self, doc: &Document) -> Result<Book> {
        let id = doc.get_object_id(constants::ID)?;
        let name = doc.get_str(constants::NAME)?;
        let author = doc.get_str(constants::AUTHOR)?;
        let num_pages = doc.get_i32(constants::NUM_PAGES)?;
        let added_at = doc.get_datetime(constants::ADDED_AT)?;
        let tags = doc.get_array(constants::TAGS)?;

        let book = Book {
            id: id.to_hex(),
            name: name.to_owned(),
            author: author.to_owned(),
            num_pages: num_pages as usize,
            added_at: added_at.to_chrono(),
            tags: tags
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
        };
        Ok(book)
    }
}
