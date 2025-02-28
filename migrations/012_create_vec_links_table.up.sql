create virtual table vec_links using vec0(
  vec_source float[384],
  vec_description float[384],
);

insert into vec_links (rowid, vec_source, vec_description)
select id, lembed(source), lembed(description)
from links;
