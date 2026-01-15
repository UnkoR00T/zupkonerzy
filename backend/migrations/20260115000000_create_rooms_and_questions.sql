CREATE TABLE IF NOT EXISTS rooms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    owner UUID NOT NULL REFERENCES clients(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    question TEXT NOT NULL,
    img_url TEXT,
    video_url TEXT,
    answers TEXT[] NOT NULL,
    correct INTEGER NOT NULL
);
