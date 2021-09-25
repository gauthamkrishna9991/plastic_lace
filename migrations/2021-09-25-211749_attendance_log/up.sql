-- Attendance Log Model

CREATE TABLE IF NOT EXISTS attendance_logs (
    id bigserial PRIMARY KEY,
    admin serial NOT NULL,
    checkin timestamp DEFAULT current_timestamp,
    checkout timestamp DEFAULT current_timestamp + interval '3 hours'
);
