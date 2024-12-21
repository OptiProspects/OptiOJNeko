#!/bin/bash

# 设置变量
IMAGE_NAME="optioj/judge"
IMAGE_VERSION="latest"

# 构建镜像
echo "Building Docker image..."
docker build -t ${IMAGE_NAME}:${IMAGE_VERSION} .

# 检查构建是否成功
if [ $? -eq 0 ]; then
    echo "Build successful!"
    
    # 询问是否要推送镜像
    read -p "Do you want to push the image to Docker Hub? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Pushing image to Docker Hub..."
        docker push ${IMAGE_NAME}:${IMAGE_VERSION}
        
        if [ $? -eq 0 ]; then
            echo "Push successful!"
        else
            echo "Push failed!"
            exit 1
        fi
    fi
else
    echo "Build failed!"
    exit 1
fi 