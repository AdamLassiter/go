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
insert into links (is_alias, target, source) values (false, 'https://bing.com', 'bing');
insert into links (is_alias, target, source) values (false, 'https://duckduckgo.com', 'duckduckgo');
insert into links (is_alias, target, source) values (false, 'https://askjeeves.com', 'askjeeves');
insert into links (is_alias, target, source) values (false, 'https://yahoo.com', 'yahoo');
insert into links (is_alias, target, source) values (false, 'https://perplexity.ai', 'perplexity');
insert into links (is_alias, target, source) values (false, 'https://you.com', 'you');
insert into links (is_alias, target, source) values (false, 'https://chatgpt.com', 'chatgpt');
insert into links (is_alias, target, source) values (false, 'https://yep.com', 'yep');
insert into links (is_alias, target, source) values (false, 'https://openverse.com', 'openverse');
insert into links (is_alias, target, source) values (false, 'https://ecosia.com', 'ecosia');
insert into links (is_alias, target, source) values (false, 'https://aol.com', 'aol');
