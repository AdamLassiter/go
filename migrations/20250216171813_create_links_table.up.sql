create table if not exists links (
  id integer primary key autoincrement,
  source text not null,
  is_alias boolean not null,
  target text not null,
  created_at datetime default current_timestamp,
  modified_at datetime default current_timestamp,
  
  unique (source) on conflict rollback
);

insert into links (is_alias, target, source) values (false, 'https://google.com', 'google');
