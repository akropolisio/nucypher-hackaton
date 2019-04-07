pg_ctl -D /usr/local/var/postgres start

# Usage

1. Send contact number to register it: `curl -S --header "Content-Type: application/json" --request POST --data '{"contact":"+1555000-00-00"}'  http://{IP}:8080/register/init`
1. Get status of registration: ``curl -S'  http://{IP}:8080//state/+1555000-00-00`

## Nu

- Prepare node (gen/import keys) like in nu-docs.
- Run node: `nucypher ursula run --network devnet --teacher-uri 18.222.119.242:9151 --federated-only`


# Sqlite3

- `echo "DATABASE_URL=file:test.db" > .env`
- `diesel setup`
- `diesel migration run`
- `diesel migration redo`

## View DB entries

```
> sqlite3 test.db
sqlite> .tables
sqlite> select * from users;
```
