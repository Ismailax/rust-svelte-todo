up:
	./merge-env.sh && docker compose up -d --build

down:
	docker compose down --rmi all --volumes --remove-orphans

rebuild-backend:
	docker compose up -d --build backend

rebuild-frontend:
	docker compose up -d --build frontend