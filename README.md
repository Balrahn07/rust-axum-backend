Run postgres in docker:
sudo docker run --name rust-postgres -e POSTGRES_USER=rust_user -e POSTGRES_PASSWORD=password -e POSTGRES_DB=rust_backend -p 5432:5432 -d postgres

To enter in psql in docker:
docker exec -it rust-postgres psql -U rust_user -d rust_backend
