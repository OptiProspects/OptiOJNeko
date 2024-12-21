@echo off
setlocal

echo Cleaning up Docker resources...

echo.
echo Stopping and removing judge containers...
docker-compose down

echo.
echo Removing unused containers...
docker container prune -f

echo.
echo Removing unused images...
docker image prune -f

echo.
echo Removing unused volumes...
docker volume prune -f

echo.
echo Removing unused networks...
docker network prune -f

echo.
echo Cleanup completed!

endlocal 