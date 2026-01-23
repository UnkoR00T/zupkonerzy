CREATE TABLE IF NOT EXISTS rooms (
    id TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    name TEXT NOT NULL,
    owner TEXT NOT NULL REFERENCES clients(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY DEFAULT gen_random_uuid()::text,
    room_id TEXT NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    question TEXT NOT NULL,
    img_url TEXT,
    video_url TEXT,
    answers TEXT[] NOT NULL,
    correct INTEGER NOT NULL
);
