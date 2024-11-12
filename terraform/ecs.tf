resource "aws_ecs_cluster" "ecs_cluster" {
 name = "${var.name}-ecs-cluster"
}

resource "aws_ecs_capacity_provider" "ecs_capacity_provider" {
 name = "${var.name}-capacity-provider"

 auto_scaling_group_provider {
   auto_scaling_group_arn = aws_autoscaling_group.ecs_asg.arn

   managed_scaling {
     maximum_scaling_step_size = 1000
     minimum_scaling_step_size = 1
     status                    = "ENABLED"
     target_capacity           = 1
   }
 }
}

resource "aws_ecs_cluster_capacity_providers" "capacity_providers" {
 cluster_name = aws_ecs_cluster.ecs_cluster.name

 capacity_providers = [aws_ecs_capacity_provider.ecs_capacity_provider.name]

 default_capacity_provider_strategy {
   base              = 1
   weight            = 100
   capacity_provider = aws_ecs_capacity_provider.ecs_capacity_provider.name
 }
}

resource "aws_ecs_task_definition" "ecs_task_definition" {
 family             = "${var.name}-ecs-task"
 network_mode       = "awsvpc"
 execution_role_arn = aws_iam_role.ecsTaskExecutionRole.arn
 cpu                = 512
 runtime_platform {
   operating_system_family = "LINUX"
   cpu_architecture        = "X86_64"
 }

 container_definitions = jsonencode([
   {
     name      = var.name
     image     = "${aws_ecr_repository.crustchan-repo.repository_url}:latest"
     cpu       = 512
     memory    = 512
     essential = true
     portMappings = [
       {
         containerPort = 3000
         hostPort      = 3000
         protocol      = "tcp"
       },
     ],
     environment = [
      {name = "AWS_ACCESS_KEY_ID", value= aws_iam_access_key.crustchan-key.id},
      {name = "AWS_ACCESS_KEY_SECRET", value=aws_iam_access_key.crustchan-key.secret},
     ]
      logConfiguration = {
        logDriver = "awslogs",
        options = {
            awslogs-group= "/ecs/crustchan-ecs-task",
            mode= "non-blocking",
            awslogs-create-group= "true",
            max-buffer-size="25m",
            awslogs-region = "us-west-2",
            awslogs-stream-prefix = "ecs"
        },
      }
    
   }
 ])
}

resource "aws_iam_access_key" "crustchan-key" {
  user    = aws_iam_user.crustchan.name
}

resource "aws_iam_user" "crustchan" {
  name = "crustchan-api"
  path = "/crustchan/"
}
resource "aws_iam_group" "crustchan-api" {
  name = "crustchan-api"
}
resource "aws_iam_user_group_membership" "crustchan" {
  user = aws_iam_user.crustchan.name

  groups = [
    aws_iam_group.crustchan-api.name,
  ]
}

resource "aws_ecs_service" "ecs_service" {
 name            = "${var.name}-ecs-service"
 cluster         = aws_ecs_cluster.ecs_cluster.id
 task_definition = aws_ecs_task_definition.ecs_task_definition.arn
 desired_count   = 2

 network_configuration {
   subnets         = [aws_subnet.public_subnet.id, aws_subnet.subnet2.id]
   security_groups = [aws_security_group.ec2_security_group.id]
 }

 force_new_deployment = true
#  placement_constraints {
#    type = "distinctInstance"
#  }

 triggers = {
   redeployment = plantimestamp()
 }

 capacity_provider_strategy {
   capacity_provider = aws_ecs_capacity_provider.ecs_capacity_provider.name
   weight            = 100
 }

 load_balancer {
   target_group_arn = aws_lb_target_group.ecs_tg.arn
   container_name   = var.name
   container_port   = 3000
 }
 depends_on = [aws_autoscaling_group.ecs_asg]
}

# resources we need for ECS

resource "aws_launch_template" "ecs_lt" {

 name_prefix   = "${var.name}-ecs-template"
 image_id      = "ami-0b7c527be879b7737"
 instance_type = "t3.micro"
 key_name      = "laptop"
 vpc_security_group_ids = [aws_security_group.ec2_security_group.id]

 iam_instance_profile {
   arn = aws_iam_instance_profile.crustchan_api_profile.arn
 }

 block_device_mappings {
   device_name = "/dev/xvda"
   ebs {
     volume_size = 30
     volume_type = "gp2"
   }
 }

 tag_specifications {
   resource_type = "instance"
   tags = {
     environment = var.environment
     name        = "${var.name}-ec2instance"
   }
 }
 user_data = filebase64("${path.module}/ecs.sh")
}

resource "aws_autoscaling_group" "ecs_asg" {
 vpc_zone_identifier = [aws_subnet.public_subnet.id, aws_subnet.subnet2.id]
 desired_capacity    = 1
 max_size            = 1
 min_size            = 1

 launch_template {
   id      = aws_launch_template.ecs_lt.id
   version = "$Latest"
 }

 tag {
   key                 = "AmazonECSManaged"
   value               = true
   propagate_at_launch = true
 }
}

resource "aws_lb" "ecs_alb" {
 name               = "ecs-alb"
 internal           = false
 load_balancer_type = "application"
 security_groups    = [aws_security_group.ec2_security_group.id]
 subnets            = [aws_subnet.public_subnet.id, aws_subnet.subnet2.id]

  tags = {
    name = var.name
    environment = var.environment
  }
}

resource "aws_lb_listener" "ecs_alb_listener" {
 load_balancer_arn = aws_lb.ecs_alb.arn
 port              = 80
 protocol          = "HTTP"

 default_action {
   type             = "forward"
   target_group_arn = aws_lb_target_group.ecs_tg.arn
 }
}

resource "aws_lb_target_group" "ecs_tg" {
 name        = "ecs-target-group"
 port        = 3000
 protocol    = "HTTP"
 target_type = "ip"
 vpc_id      = aws_vpc.vpc.id

 health_check {
   path = "/health"
 }
}