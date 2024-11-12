
resource "aws_iam_role" "api_server_role" {
  name = "api_server_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Sid    = ""
        Principal = {
          Service = "ec2.amazonaws.com"
        }
      },
    ]
  })
}

resource "aws_iam_instance_profile" "crustchan_api_profile" {
  name = "crustchan-api-profile"
  role = aws_iam_role.api_server_role.name
}

data "aws_iam_policy" "rekognition_service_role" {
  arn = "arn:aws:iam::aws:policy/service-role/AmazonRekognitionServiceRole"
}
data "aws_iam_policy" "s3_read_access" {
  arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess"
}
data "aws_iam_policy" "dynamodb_access" {
  arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaDynamoDBExecutionRole"
}
data "aws_iam_policy" "ecs_task_execution" {
  arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

data "aws_iam_policy" "s3_put_object_access" {
  arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess"
}

resource "aws_iam_role_policy_attachment" "api_dynamodb_service_role_policy_attach" {
  role       = aws_iam_role.api_server_role.name
  policy_arn = data.aws_iam_policy.dynamodb_access.arn

}
resource "aws_iam_role_policy_attachment" "api_s3_service_role_policy_attach" {
  role       = aws_iam_role.api_server_role.name
  policy_arn = data.aws_iam_policy.s3_read_access.arn
}


resource "aws_iam_role" "app_lambda_role" {
  name = "lambda-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Sid    = ""
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      },
    ]
  })
}
resource "aws_iam_role_policy_attachment" "lambda_rekognition_service_role_policy_attach" {
  role       = aws_iam_role.app_lambda_role.name
  policy_arn = data.aws_iam_policy.rekognition_service_role.arn
}

resource "aws_iam_role_policy_attachment" "lambda_dynamodb_service_role_policy_attach" {
  role       = aws_iam_role.app_lambda_role.name
  policy_arn = data.aws_iam_policy.dynamodb_access.arn
}



resource "aws_iam_role" "ecsTaskExecutionRole" {
  name = "ecsTaskExecutionRole"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Sid    = ""
        Principal = {
          Service = ["ec2.amazonaws.com",
            "ecs-tasks.amazonaws.com",
          "ecs.amazonaws.com"]
        }
      },
    ]
  })
}
resource "aws_iam_role_policy_attachment" "ecs_service_role_policy_attach" {
  role       = aws_iam_role.ecsTaskExecutionRole.name
  policy_arn = data.aws_iam_policy.ecs_task_execution.arn
}