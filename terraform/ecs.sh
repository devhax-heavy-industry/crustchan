#!/bin/bash
mkdir -p /etc/ecs
echo ECS_CLUSTER=crustchan-ecs-cluster >> /etc/ecs/ecs.config
docker stop ecs-agent
docker rm ecs-agent
docker pull amazon/amazon-ecs-agent:latest
sudo service docker restart && sudo start ecs