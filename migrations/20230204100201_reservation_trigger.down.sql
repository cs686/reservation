-- Add down migration script here
DROP TABLE rsvp.reservation_changes;
DROP TRIGGER reservations_trigger on rsvp.reservations;
DROP FUNCTION rsvp.reservations_trigger;
