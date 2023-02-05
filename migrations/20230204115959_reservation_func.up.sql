-- Add up migration script here
CREATE OR REPLACE FUNCTION rsvp.query(uid text, rid text, during tstzrange) RETURNS TABLE (LIKE rsvp.reservations)
AS $$
BEGIN
    IF uid IS NULL AND rid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations where during && timespan;
    ELSIF uid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations where resource_id = rid AND during @> timespan;
    ELSIF rid IS NULL THEN
        RETURN QUERY SELECT * FROM rsvp.reservations where user_id = uid AND during @> timespan;
    ELSE
        RETURN QUERY SELECT * FROM rsvp.reservations where user_id = uid AND resource_id = rid AND during @> timespan;
    END IF;
END
$$ LANGUAGE plpgsql;
