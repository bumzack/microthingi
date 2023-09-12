

sudo -i -u postgres

/Library/PostgreSQL/15/bin/createuser --interactive -p 5433

microthingi


/Library/PostgreSQL/15/bin/psql  -p 5433

\du


\password microthingi 
 

diesel setup

    






DROP database microthingi;
REVOKE ALL ON SCHEMA public FROM  microthingi;
DROP role microthingiuser;



DROP database microthingi;
REVOKE ALL ON SCHEMA public FROM  microthingi;
DROP role microthingi;






drop table __diesel_schema_migrations;
drop table articles;
drop table images;
drop table prices;
drop table texts;

