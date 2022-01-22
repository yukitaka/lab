drop table if exists artists;
create table artists (
  constituent_id integer primary key,
  display_name varchar(256) not null,
  artist_bio varchar(256),
  nationality varchar(128),
  gender varchar(16),
  begin_date integer not null,
  end_date integer default 0 not null,
  wiki_qid varchar(16),
  ulan integer
);
\copy artists from artists.csv with csv header;
