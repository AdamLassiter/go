create trigger if not exists vec_links_inserter
after insert on links
begin
  insert into vec_links (rowid, vec_source, vec_description)
  values (new.id, lembed(new.source), lembed(new.description));
end;

create trigger if not exists vec_links_updater
after update on links
begin
  update vec_links
  set (vec_source, vec_description)
  = (lembed(new.source), lembed(new.description))
  where rowid = old.id;
end;

create trigger if not exists vec_links_deleter
after delete on links
begin
  delete from vec_links
  where rowid = old.id;
end;
