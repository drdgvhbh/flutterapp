use super::schema::{deltas, document_histories, documents};
use chrono::NaiveDateTime;
use diesel::prelude::QueryResult;

#[derive(Clone, Debug, PartialEq, Eq, Queryable, Identifiable, AsChangeset, Associations)]
#[belongs_to(Delta, foreign_key = "id")]
#[table_name = "documents"]
pub struct Document {
    pub id: String,
    pub created_at: NaiveDateTime,
}

impl Document {
    pub fn delta(&self, conn: &diesel::PgConnection) -> QueryResult<Delta> {
        use crate::diesel::*;

        let latest_history = document_histories::table
            .select(document_histories::table::all_columns())
            .filter(document_histories::document_id.eq(&self.id))
            .order(document_histories::created_at.desc())
            .first::<DocumentHistory>(&*conn)?;
        Ok(deltas::table
            .select(deltas::table::all_columns())
            .filter(deltas::id.eq(&latest_history.delta_id))
            .first::<Delta>(&*conn)?)
    }
}

#[derive(Insertable)]
#[table_name = "documents"]
pub struct NewDocument<'a> {
    pub id: &'a str,
}

impl<'a> NewDocument<'a> {
    pub fn new(id: &'a str) -> Self {
        NewDocument { id: &id }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Queryable, Identifiable, AsChangeset, Associations)]
#[table_name = "document_histories"]
#[belongs_to(Delta, foreign_key = "id")]
#[belongs_to(Document, foreign_key = "id")]
pub struct DocumentHistory {
    pub id: i32,
    pub document_id: String,
    pub delta_id: String,
    pub created_at: NaiveDateTime,
}

impl DocumentHistory {
    pub fn delta(&self, conn: &diesel::PgConnection) -> QueryResult<Delta> {
        use crate::diesel::*;

        Ok(deltas::table
            .select(deltas::table::all_columns())
            .filter(deltas::id.eq(&self.delta_id))
            .first::<Delta>(&*conn)?)
    }

    pub fn document(&self, conn: &diesel::PgConnection) -> QueryResult<Document> {
        use crate::diesel::*;

        Ok(documents::table
            .select(documents::table::all_columns())
            .filter(documents::id.eq(&self.document_id))
            .first::<Document>(&*conn)?)
    }
}

#[derive(Insertable)]
#[table_name = "document_histories"]
pub struct NewDocumentHistory {
    pub document_id: String,
    pub delta_id: String,
}

impl NewDocumentHistory {
    pub fn new(document: &Document, delta: &Delta) -> Self {
        NewDocumentHistory {
            document_id: document.id.clone(),
            delta_id: delta.id.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Queryable, Identifiable, AsChangeset, Associations)]
#[table_name = "deltas"]
pub struct Delta {
    pub id: String,
    pub delta: String,
}

#[derive(Insertable)]
#[table_name = "deltas"]
pub struct NewDelta<'a> {
    pub id: String,
    pub delta: &'a str,
}

impl<'a> NewDelta<'a> {
    pub fn new(delta: &'a str) -> Self {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.input(delta);
        let hash_bytes = &hasher.result()[..];

        NewDelta {
            id: hex::encode(&hash_bytes),
            delta,
        }
    }
}
