-- CREATE TABLE poststhree (
--   id INTEGER AUTO_INCREMENT PRIMARY KEY,
--   title VARCHAR(255) NOT NULL,
--   body TEXT NOT NULL,
--   published BOOLEAN NOT NULL DEFAULT FALSE
-- );

-- CREATE TABLE posts (
--   id INTEGER NOT NULL PRIMARY KEY AUTO_INCREMENT,
--   title VARCHAR(255) NOT NULL,
--   body TEXT NOT NULL,
--   published BOOLEAN NOT NULL DEFAULT 0
-- );

-- CREATE TABLE errtype (
--   id INTEGER PRIMARY KEY AUTO_INCREMENT,
--   name VARCHAR(255) NOT NULL,
--   identifier TEXT NOT NULL
-- );

CREATE TABLE errors (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  matching_string VARCHAR(255) NOT NULL,
  reference_url TEXT NOT NULL,
  reference_case TEXT NOT NULL
);

CREATE TABLE whitelist (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  matching_string VARCHAR(255) NOT NULL,
  reference_url TEXT NOT NULL,
  reference_case TEXT NOT NULL
)

