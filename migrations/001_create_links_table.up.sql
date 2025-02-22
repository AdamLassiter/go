create table if not exists links (
  id integer primary key autoincrement,
  created_at datetime default current_timestamp,
  modified_at datetime default current_timestamp,

  source text not null,
  is_alias boolean not null,
  target text not null,
  description text default source,
  
  unique (source) on conflict rollback
);
