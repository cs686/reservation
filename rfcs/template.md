# Feature
- Feature Name: (fill me in with a unique ident, `my_awesome_feature`)
- Start Date: (fill me in with today's date, YYYY-MM-DD)
- RFC PR: [rust-lang/rfcs#0000](https://github.com/rust-lang/rfcs/pull/0000)
- Rust Issue: [rust-lang/rust#0000](https://github.com/rust-lang/rust/issues/0000)

# Summary
[summary]: #summary

One paragraph explanation of the feature.

# Motivation
[motivation]: #motivation

Why are we doing this? What use cases does it support? What is the expected outcome?

# Guide-level explanation
[guide-level-explanation]: #guide-level-explanation

## Service interface

```proto
    enum ReservationStatus {
        UNKNOW = 0;
        PENDING = 1;
        CONFIRMED = 2;
        CONCEDE  = 3;
    }

    enum ReservationUpdateType {
        UNKNOW = 0;
        INSERT = 1;
        UPDATE = 2;
        DELETE  = 3;
    }

    message Reservation {
        string id = 1;
        string user_id = 2;
        ReservationStatus status = 3;

        string resource_id = 4;
        google.protobuf.Timestamp start =5;
        google.protobuf.Timestamp end =6;

        string note =7;
    }

    message ReserveRequest {
        Reservation reservation = 1;
    }

    message ReserveResponse {
        Reservation reservation = 1;
    }

    message UpdateRequest {
        string note = 1;
    }

    message UpdateResponse {
        Reservation reservation = 1;
    }

    message UpdateRequest {
        string note = 1;
    }

    message UpdateResponse {
        Reservation reservation = 1;
    }

    message ConfirmRequest {
        string id = 1;
    }

    message ConfirmResponse {
        Reservation reservation = 1;
    }

    message CancelRequest {
        string id = 1;
    }

    message CancelResponse {
        Reservation reservation = 1;
    }

    message GetRequest {
        string id = 1;
    }

    message GetResponse {
        Reservation reservation = 1;
    }

    message QueryRequest {
        string resource_id = 1;
        string user_id = 2;
        google.protobuf.Timestamp start =3;
        google.protobuf.Timestamp end =4;
    }

    message ListenRequest {

    }

    message ListenResponse {
        int8 op = 1;
        Reservation reservation = 2;
    }

    service ReservationService {
        rpc reserve(ReserveRequest) returns (ReserveResponse);
        rpc update(UpdateRequest) returns (UpdateResponse);
        rpc confirm(ConfirmRequest) returns (ConfirmResponse);
        rpc cancel(CancelRequest) returns (CancelResponse);
        rpc get(GetRequest) returns (GetResponse);
        rpc query() returns (stream Reservation);
        rpc listen(ListenRequest) returns (stream ListenResponse);
    }
```
## Database Schema

```sql
CREATE SCHEMA rsvp;

CREATE TYPE rsvp.reservation_status AS ENUM('unknown','pending', 'confirmed', 'canceled')
CREATE TYPE rsvp.reservation_update_type AS ENUM('unknown','insert', 'update', 'delete')

CREATE TABLE rsvp.reservations (
    id uuid NOT NULL default uuid_generate_v4(),
    user_id varchar(64) NOT NULL ,
    status reservation_status NOT NULL default 'pending',
    resource_id varchar(64) NOT NULL,
    timespan tstzrange NOT NULL,
    note text,

    CONSTRAINT reservations_pkey PRIMARY KEY (id),
    CONSTRAINT reservations_conflict EXCLUDE USING gist (resource_id WITH =, timespan WITH &&)
);

CREATE TABLE rsvp.reservation_changes (
    id SERIAL NOT NULL ,
    reservation_id uuid NOT NULL ,
    op rsvp.reservation_update_type NOT NULL
);

CREATE INDEX reservations_resource_id_idx ON rsvp.reservations (resource_id);
CREATE INDEX reservations_user_id_idx ON rsvp.reservations (user_id);
CREATE OR REPLACE FUNCTION rsvp.query(uid text, rid text, during: tstzrange)
RETURNS TABLE rsvp.reservations AS $$ $$ LANGUAGE plpgsql;
CREATE OR REPLACE FUNCTION rsvp.reservations_trigger() RETURNS TRIGGER AS
$$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'create'),
    ELSE IF TG_OP = 'UPDATE' THEN
        IF OLD.status <> NEW.status THEN
            INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'update'),
        END IF;
    ELSE IF TG_OP = 'DELETE' THEN
        INSERT INTO rsvp.reservation_changes (reservation_id, op) VALUES (NEW.id, 'delete'),
    END IF;
    NOTIFY reservation_update;
    RETURN NULL;
END ;
$$ LANGUAGE plpgsql;
CREATE TRIGGER reservations_trigger
    AFTER INSERT OR UPDATE OR DELETE ON rsvp.reservations FOR EACH ROW EXECUTE PROCEDURE rsvp.reservations_trigger();
```


# Reference-level explanation
[reference-level-explanation]: #reference-level-explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

# Drawbacks
[drawbacks]: #drawbacks

Why should we *not* do this?

# Rationale and alternatives
[rationale-and-alternatives]: #rationale-and-alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?
- If this is a language proposal, could this be done in a library or macro instead? Does the proposed change make Rust code easier or harder to read, understand, and maintain?

# Prior art
[prior-art]: #prior-art

Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

- For language, library, cargo, tools, and compiler proposals: Does this feature exist in other programming languages and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- For other teams: What lessons can we learn from what other communities have done here?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to, this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other languages, provide readers of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an adaptation from other languages.

Note that while precedent set by other languages is some motivation, it does not on its own motivate an RFC.
Please also take into consideration that rust sometimes intentionally diverges from common language features.

# Unresolved questions
[unresolved-questions]: #unresolved-questions

- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?

# Future possibilities
[future-possibilities]: #future-possibilities

Think about what the natural extension and evolution of your proposal would
be and how it would affect the language and project as a whole in a holistic
way. Try to use this section as a tool to more fully consider all possible
interactions with the project and language in your proposal.
Also consider how this all fits into the roadmap for the project
and of the relevant sub-team.

This is also a good place to "dump ideas", if they are out of scope for the
RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities,
you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section
is not a reason to accept the current or a future RFC; such notes should be
in the section on motivation or rationale in this or subsequent RFCs.
The section merely provides additional information.
