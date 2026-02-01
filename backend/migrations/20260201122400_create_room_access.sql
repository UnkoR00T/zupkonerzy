-- Add migration script here
CREATE TABLE room_access (
    room_id TEXT NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    client_id TEXT NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    PRIMARY KEY (room_id, client_id)
);
