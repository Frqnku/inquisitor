build:
	docker-compose build

up:
	docker-compose up -d
	@echo "Retrieving IP and MAC addresses..."
	@echo "FTP Server" > info.txt
	@docker network inspect inquisitor-net \
		| sed -n '/"Name": "ftp_server"/,/}/p' \
		| grep -E '"IPv4Address"|"MacAddress"' \
		| awk -F'"' '{ip=$$4; gsub("/16","",ip); print ip}' \
		>> info.txt
	@echo "" >> info.txt
	@echo "FTP Client" >> info.txt
	@docker network inspect inquisitor-net \
		| sed -n '/"Name": "ftp_client"/,/}/p' \
		| grep -E '"IPv4Address"|"MacAddress"' \
		| awk -F'"' '{ip=$$4; gsub("/16","",ip); print ip}' \
		>> info.txt

	@echo "Written to info.txt :"
	@cat info.txt

down:
	docker-compose down

clean:
	docker-compose down --rmi all -v

logs:
	docker-compose logs -f

restart: down up

.PHONY: build up down clean logs restart
