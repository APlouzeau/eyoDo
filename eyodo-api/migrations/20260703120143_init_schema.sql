CREATE TABLE users(
   id SERIAL,
   name VARCHAR(50) NOT NULL,
   password CHAR(60) NOT NULL,
   PRIMARY KEY(id)
);

CREATE TABLE group_(
   id SERIAL,
   name VARCHAR(50) NOT NULL,
   PRIMARY KEY(id)
);

CREATE TABLE group_members(
   group_id INTEGER,
   user_id INTEGER,
   PRIMARY KEY(group_id, user_id),
   FOREIGN KEY(group_id) REFERENCES group_(id),
   FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE todos(
   id SERIAL,
   title VARCHAR(255) NOT NULL,
   description TEXT,
   due_date DATE,
   completed_at TIMESTAMP,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   creator_id INTEGER NOT NULL,
   owner_user_id INTEGER,
   owner_group_id INTEGER,
   PRIMARY KEY(id),
   FOREIGN KEY(creator_id) REFERENCES users(id),
   FOREIGN KEY(owner_user_id) REFERENCES users(id),
   FOREIGN KEY(owner_group_id) REFERENCES group_(id),
   CHECK (
     (owner_user_id IS NOT NULL AND owner_group_id IS NULL) OR
     (owner_user_id IS NULL AND owner_group_id IS NOT NULL)
   )
);

CREATE TABLE todo_assignees(
   todo_id INTEGER,
   user_id INTEGER,
   PRIMARY KEY(todo_id, user_id),
   FOREIGN KEY(todo_id) REFERENCES todos(id),
   FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE comment(
   id SERIAL,
   todo_id INTEGER NOT NULL,
   author_id INTEGER NOT NULL,
   comment TEXT NOT NULL,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   PRIMARY KEY(id),
   FOREIGN KEY(todo_id) REFERENCES todos(id),
   FOREIGN KEY(author_id) REFERENCES users(id)
);