# Wingstats API

Instructions for testing:

- `curl -X POST -H "Content-Type: application/json" -d '{"someone": "60", "arsh": "110"}' http://localhost:3000/`
- `curl -X POST -H "Content-Type: application/json" -d '{"player_id": 123, "name": "arsh"}' http://localhost:3000/api/v1/players`

Connect to running mysql DB:

- `docker exec -it 083671142ba4 bash`
- `mysql -u admin -p -D wingstats`
- `SHOW TABLES;` `DESCRIBE players;`