@echo off
setlocal

set IMAGE_NAME=optioj/judge
set IMAGE_VERSION=latest

echo Building Docker image...
docker-compose --progress=plain --verbose build

if %ERRORLEVEL% EQU 0 (
    echo Build successful!
    
    set /p "PUSH=Do you want to push the image to Docker Hub? (y/n): "
    if /i "%PUSH%"=="y" (
        echo Pushing image to Docker Hub...
        docker push %IMAGE_NAME%:%IMAGE_VERSION%
        
        if %ERRORLEVEL% EQU 0 (
            echo Push successful!
        ) else (
            echo Push failed!
            exit /b 1
        )
    )
) else (
    echo Build failed!
    exit /b 1
)

endlocal 