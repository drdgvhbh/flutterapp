use diesel::pg::PgConnection;
use std::sync::Mutex;

use diesel::prelude::*;
use diesel::result::Error;
use std::ops::Deref;

use super::schema::{deltas, document_histories, documents};

use super::models;

pub struct Context {
    pub database: Mutex<PgConnection>,
}

impl juniper::Context for Context {}

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct DocumentInput {
    pub id: String,
}

pub struct Query;

#[juniper::object(
    Context = Context,
    Scalar = juniper::DefaultScalarValue,
)]
impl Query {
    pub fn apiVersion() -> &str {
        "1.0"
    }

    pub fn document(context: &Context, input: DocumentInput) -> juniper::FieldResult<String> {
        let conn_guard = context.database.lock().unwrap();
        let conn = conn_guard.deref();

        Ok(conn.transaction::<_, Error, _>(|| {
            let document: models::Document = documents::table
                .filter(documents::dsl::id.eq(input.id))
                .get_result(conn)?;
            let delta = document.delta(conn)?;
            Ok(delta.delta)
        })?)
    }
}

#[derive(juniper::GraphQLInputObject, Debug)]
pub struct ChangeInput {
    pub id: String,
    pub parent_hash: String,
    pub change: String,
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    pub fn newDocument(context: &Context) -> juniper::FieldResult<String> {
        use uuid::Uuid;
        let id = Uuid::new_v4().to_string();
        let delta = quill_delta::Delta::new().insert("\n", quill_delta::none());
        let serialized_delta = serde_json::to_string(&delta)?;

        let new_delta = models::NewDelta::new(&serialized_delta);

        let conn_guard = context.database.lock().unwrap();
        let conn = conn_guard.deref();
        Ok(conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(deltas::table)
                .values(&new_delta)
                .on_conflict_do_nothing()
                .execute(conn)?;
            let upserted_delta = deltas::table
                .filter(deltas::id.eq(new_delta.id))
                .get_result(conn)?;

            let id = Uuid::new_v4().to_string();
            let document: models::Document = diesel::insert_into(documents::table)
                .values(models::NewDocument::new(&id))
                .get_result(conn)?;

            diesel::insert_into(document_histories::table)
                .values(models::NewDocumentHistory::new(&document, &upserted_delta))
                .execute(conn)?;
            Ok(document.id)
        })?)
    }

    pub fn change(context: &Context, input: ChangeInput) -> juniper::FieldResult<bool> {
        let delta: quill_delta::Delta = serde_json::from_str(&input.change)?;

        let conn_guard = context.database.lock().unwrap();
        let conn = conn_guard.deref();

        conn.transaction::<_, Error, _>(|| {
            let document_history: models::DocumentHistory = document_histories::table
                .select(document_histories::table::all_columns())
                .filter(document_histories::document_id.eq(input.id))
                .filter(document_histories::delta_id.eq(input.parent_hash))
                .order(document_histories::created_at.desc())
                .get_result(conn)?;

            let serialized_server_delta = document_history.delta(conn)?.delta;
            let server_delta: quill_delta::Delta =
                serde_json::from_str(&serialized_server_delta).unwrap();

            println!("incoming {}", input.change);
            // server_delta.transform(other: &Delta, priority: bool);
            println!("current {}", serde_json::to_string(&server_delta).unwrap());
            Ok(())
        })?;

        // context.document.compose(&delta);
        // println!("deserialized = {:#?}", delta);
        Ok(true)
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
