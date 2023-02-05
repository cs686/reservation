use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};

use abi::reservation::{Reservation, ReservationQuery};
use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::postgres::types::PgRange;
use sqlx::Row;

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: Reservation) -> Result<Reservation, ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }

        let start = rsvp.start.as_ref().unwrap();
        let start = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(start.seconds, start.nanos as u32).unwrap(),
            Utc,
        );

        let end = rsvp.end.as_ref().unwrap();
        let end = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(end.seconds, end.nanos as u32).unwrap(),
            Utc,
        );
        if start <= end {
            return Err(ReservationError::InvalidTime);
        }

        let timespan: PgRange<DateTime<Utc>> = (start..end).into();

        let sql = "INSERT INTO reservation (user_id, resource_id, timespan, note,status) VALUES \
        ($1, $2, $3, $4, $5) RETURNING id";
        let id = sqlx::query(sql)
            .bind(rsvp.user_id.clone())
            .bind(rsvp.resource_id.clone())
            .bind(timespan)
            .bind(rsvp.note.clone())
            .bind(rsvp.status)
            .fetch_one(&self.pool)
            .await?
            .get(0);

        rsvp.id = id;
        Ok(rsvp)
    }

    async fn change_status(&self, _id: ReservationId) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn update_note(
        &self,
        _id: ReservationId,
        _note: String,
    ) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn delete(&self, _id: ReservationId) -> Result<(), ReservationError> {
        todo!()
    }

    async fn get(&self, _id: ReservationId) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn query(&self, _query: ReservationQuery) -> Result<Vec<Reservation>, ReservationError> {
        todo!()
    }
}
