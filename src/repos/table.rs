use std::marker::PhantomData;
use std::sync::Arc;
use sqlx::{FromRow, PgPool};
use sqlx::postgres::PgRow;

#[derive(Debug)]
pub struct Table<'a, T>
    where
        T: FromRow<'a, PgRow> {
    pub pool: Arc<PgPool>,

    _from_row: fn(&'a PgRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'a T>,
}

impl<'c, T> Table<'c, T>
    where
        T: FromRow<'c, PgRow>,
{
    pub fn new(pool: Arc<PgPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}