CREATE TABLE errors (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  matching_string VARCHAR(255) NOT NULL UNIQUE,
  reference_url TEXT NOT NULL,
  reference_case TEXT NOT NULL
);
CREATE TABLE whitelist (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  matching_string VARCHAR(255) NOT NULL UNIQUE,
  reference_url TEXT NOT NULL,
  reference_case TEXT NOT NULL
);
CREATE TABLE diagnostic_files (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  diag_url VARCHAR(255) NOT NULL UNIQUE,
  sr VARCHAR(255)
);
CREATE TABLE error_results (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  matching_string_id INTEGER NOT NULL,
  diagnostic_file_id INTEGER NOT NULL,
  count INTEGER NOT NULL,
  CONSTRAINT diag_match UNIQUE (matching_string_id,diagnostic_file_id)
);