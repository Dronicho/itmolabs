-- Your SQL goes here
CREATE TABLE IF NOT EXISTS messages(
  id INTEGER PRIMARY KEY NOT NULL,
  "from" TEXT NOT NULL,
  "to" TEXT NOT NULL,
  message TEXT NOT NULL,
  CONSTRAINT fk_sender_id FOREIGN KEY ("from") REFERENCES users(email),
  CONSTRAINT fk_recipient_email FOREIGN KEY ("to") REFERENCES users(email)
);