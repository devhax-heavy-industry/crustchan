resource "aws_ecs_cluster" "ecs_cluster" {
 name = "${var.name}-ecs-cluster"
}

resource "aws_ecs_capacity_provider" "ecs_capacity_provider" {
 name = "${var.name}-capacity-provider"

 auto_scaling_group_provider {
   auto_scaling_group_arn = aws_autoscaling_group.ecs_asg.arn

   managed_scaling {
     maximum_scaling_step_size = 5
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
 cpu                = 256 
 runtime_platform {
   operating_system_family = "LINUX"
   cpu_architecture        = "X86_64"
 }

 container_definitions = jsonencode([
   {
     name      = var.name
     image     = "${aws_ecr_repository.docker_repo.repository_url}/${var.name}:latest"
     cpu       = 256
     memory    = 512
     essential = true
     portMappings = [
       {
         containerPort = 3000
         hostPort      = 3000
         protocol      = "tcp"
       }
     ]
   }
 ])
}

resource "aws_ecs_service" "ecs_service" {
 name            = "${var.name}-ecs-service"
 cluster         = aws_ecs_cluster.ecs_cluster.id
 task_definition = aws_ecs_task_definition.ecs_task_definition.arn
 desired_count   = 1

 network_configuration {
   subnets         = [aws_subnet.public_subnet.id, aws_subnet.subnet2.id]
   security_groups = [aws_security_group.ec2_security_group.id]
 }

 force_new_deployment = true
 placement_constraints {
   type = "distinctInstance"
 }

 triggers = {
   redeployment = timestamp()
 }

 capacity_provider_strategy {
   capacity_provider = aws_ecs_capacity_provider.ecs_capacity_provider.name
   weight            = 100
 }

 load_balancer {
   target_group_arn = aws_lb_target_group.ecs_tg.arn
   container_name   = "dockergs"
   container_port   = 80
 }
 depends_on = [aws_autoscaling_group.ecs_asg]
}

# resources we need for ECS

resource "aws_launch_template" "ecs_lt" {

 name_prefix   = "${var.name}-ecs-template"
 image_id      = "ami-001651dd1b19ebcb6"
 instance_type = "t3.micro"
 key_name               = "ec2ecsglog"
 vpc_security_group_ids = [aws_security_group.ec2_security_group.id]

 iam_instance_profile {
   name = "ecsInstanceRole"
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