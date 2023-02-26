create article set
  created_at = time::now(),
  title = 'Example',
  text = 'Body',
  account = (select id from account where name = 'Takayuki Sato' limit 1)
;
