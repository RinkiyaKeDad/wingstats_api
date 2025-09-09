# Wingstats API

Instructions for testing:

- `curl -X POST -H "Content-Type: application/json" -d '{"someone": "60", "arsh": "110"}' http://localhost:3000/`
- `curl -X POST -H "Content-Type: application/json" -d '{"name": "yolo"}' http://localhost:3000/api/v1/players`
- `curl -X GET http://localhost:3000/api/v1/players`
- `curl -X GET http://localhost:3000/api/v1/players/55d28232-f10c-4a4f-89c5-2158cad7d1eb`

Connect to running mysql DB:

- `docker exec -it 083671142ba4 bash`
- `mysql -u admin -p -D wingstats`
- `SHOW TABLES;` `DESCRIBE players;`

Connect to running Postgres DB:

- `docker exec -it 84bc3e20ec8d`
- `psql -U admin -d wingstats`
- `\dt` -> Show Tables
- `\d players` -> Describe players
- `SELECT * from players;` -> works for both DBs