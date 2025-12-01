CREATE TABLE user_permissions(
  uuid UUID PRIMARY KEY,
  root BOOL NOT NULL,
  manage_users BOOL NOT NULL,
  login BOOL NOT NULL
);
