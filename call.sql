CREATE TABLE calls (
    id SERIAL PRIMARY KEY,
    caller_id INT NOT NULL,
    receiver_id INT NOT NULL,
    start_time TIMESTAMP DEFAULT NOW(),
    end_time TIMESTAMP,
    status VARCHAR(20) NOT NULL, -- e.g., "initiated", "ongoing", "completed", "missed"
    FOREIGN KEY (caller_id) REFERENCES users (id),
    FOREIGN KEY (receiver_id) REFERENCES users (id)
);
