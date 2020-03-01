
< put more stuff here later >


*PUSH
curl --header "Content-Type: application/json"  --request UPDATE   --data '{ "stackname" : "abc" , "data" : "xyz" }'    http://localhost:7878/api/stack

INSERT INTO storage (stackname, data) VALUES ('abc', '123');
--

*POP
curl --header "Content-Type: application/json"  --request GET --data '{"stackname":"abc"}'  http://localhost:7878/api/stack

SELECT stackname, data, max(rowid) FROM storage WHERE stackname = 'abc';
DELETE FROM storage WHERE rowid = (SELECT max(rowid) FROM storage) AND stackname = 'abc';

--
*PEEK (Doesn't work yet)
curl --header "Content-Type: application/json"  --request GET --data '{"stackname":"abc"}'  http://localhost:7878/api/stack/peek

SELECT data FROM storage WHERE stackname = 'abc' ORDER BY rowid DESC;



[![Run on Repl.it](https://repl.it/badge/github/TonyHsieh/rustack)](https://repl.it/github/TonyHsieh/rustack)