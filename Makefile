SERVICES = ftp_server ftp_client inquisitor

build:
	docker-compose build

up:
	docker-compose up -d

down:
	docker-compose down

clean:
	docker-compose down --rmi all -v

logs:
	docker-compose logs -f

restart: down up

.PHONY: build up down clean logs restart
