create trigger if not exists vec_links_inserter
after insert on links
begin
  insert into vec_links (rowid, vec_source, vec_description)
  values (new.id, lembed('minilm', new.source), lembed('minilm', new.description));
end;

create trigger if not exists vec_links_updater
after update on links
begin
  update vec_links
  set (rowid, vec_source, vec_description)
  = (new.id, lembed('minilm', new.source), lembed('minilm', new.description))
  where rowid = old.id;
end;

create trigger if not exists vec_links_deleter
after delete on links
begin
  delete from vec_links
  where rowid = old.id;
end;
