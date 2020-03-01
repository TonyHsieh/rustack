
< put more stuff here later >


PUSH
curl --header "Content-Type: application/json"  --request UPDATE   --data '{"stack":"xyz","data":"xyz"}'    http://localhost:7878/api/stack

insert into storage (stackname, data) values ('abc', '123');


POP
curl --header "Content-Type: application/json"  --request GET --data '{"stack":"xyz"}'  http://localhost:7878/api/stack

delete from storage where rowid = (select min(rowid) and stackname = 'abc' from storage);


PEEK 
curl --header "Content-Type: application/json"  --request GET --data '{"stack":"xyz"}'  http://localhost:7878/api/stack/peek

select data from storage where stackname = 'abc';
