@echo off
setlocal

if "%1"=="" (
    echo Usage: %0 [start^|stop^|restart^|status]
    exit /b 1
)

if "%1"=="start" (
    echo Starting judge service...
    docker-compose up -d
    if %ERRORLEVEL% EQU 0 (
        echo Judge service started successfully
    ) else (
        echo Failed to start judge service
        exit /b 1
    )
)

if "%1"=="stop" (
    echo Stopping judge service...
    docker-compose down
    if %ERRORLEVEL% EQU 0 (
        echo Judge service stopped successfully
    ) else (
        echo Failed to stop judge service
        exit /b 1
    )
)

if "%1"=="restart" (
    echo Restarting judge service...
    docker-compose restart
    if %ERRORLEVEL% EQU 0 (
        echo Judge service restarted successfully
    ) else (
        echo Failed to restart judge service
        exit /b 1
    )
)

if "%1"=="status" (
    echo Judge service status:
    docker-compose ps
)

endlocal 