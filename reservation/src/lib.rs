use abi::reservation::{Reservation, ReservationQuery};
mod error;
mod manager;
use async_trait::async_trait;
use error::ReservationError;
use sqlx::PgPool;

type ReservationId = String;

#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}

#[async_trait]
pub trait Rsvp {
    async fn reserve(&self, rsvp: Reservation) -> Result<Reservation, ReservationError>;
    async fn change_status(&self, id: ReservationId) -> Result<Reservation, ReservationError>;
    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<Reservation, ReservationError>;
    async fn delete(&self, id: ReservationId) -> Result<(), ReservationError>;
    async fn get(&self, id: ReservationId) -> Result<Reservation, ReservationError>;
    async fn query(&self, query: ReservationQuery) -> Result<Vec<Reservation>, ReservationError>;
}
