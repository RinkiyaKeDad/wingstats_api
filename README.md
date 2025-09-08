# Wingstats API

Instructions for testing:

- `curl -X POST -H "Content-Type: application/json" -d '{"someone": "60", "arsh": "110"}' http://localhost:3000/`
- `curl -X POST -H "Content-Type: application/json" -d '{"name": "shaamik"}' http://localhost:3000/api/v1/players`
- `curl -X GET http://localhost:3000/api/v1/players`
- `curl -X GET http://localhost:3000/api/v1/players/42496d67-466b-4eb7-896c-2549ee89320a`

Connect to running mysql DB:

- `docker exec -it 083671142ba4 bash`
- `mysql -u admin -p -D wingstats`
- `SHOW TABLES;` `DESCRIBE players;`