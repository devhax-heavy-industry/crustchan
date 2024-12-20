# resource "aws_instance" "ec2" {

#   ami                         = "ami-001651dd1b19ebcb6" # ubuntu 24.04
#   instance_type               = "t2.micro"

#   subnet_id                   = aws_subnet.public_subnet.id
#   vpc_security_group_ids      = [aws_security_group.ec2_security_group.id]
#   associate_public_ip_address = true

#   key_name                    = aws_key_pair.ec2_key_pair.key_name
#   iam_instance_profile        = "${aws_iam_instance_profile.crustchan_api_profile.name}"


#   tags = {
#     name = var.name
#     environment = var.environment
#   }
# }


resource "aws_security_group" "ec2_security_group" {
  name        = "cc-security-group"
  description = "security group for ec2 instances"

  vpc_id = aws_vpc.vpc.id

  ingress {
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
    ingress {
    from_port   = 3000
    to_port     = 3000
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    environment = var.environment
    Name = "cc-security-group"
  }
}

resource "aws_key_pair" "ec2_key_pair" {
  key_name   = "free-tier-ec2-key"
  public_key = file("ssh-key.pub")
}

resource "aws_s3_bucket" "app_resources" {
  bucket = "crustchan-resources"

  tags = {
    name        = var.name
    environment = var.environment
  }
}

resource "aws_s3_bucket_policy" "allow_from_rekognition" {
  bucket = aws_s3_bucket.app_resources.id

  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Sid" : "Allow Rekognition",
        "Effect" : "Allow",
        "Principal" : {
          "Service" : "rekognition.amazonaws.com"
        },
        "Action" : "s3:*",
        "Resource" : "arn:aws:s3:::crustchan-resources"
      },
      {
        "Sid" : "allow-account-access-from-api-server",
        "Effect" : "Allow",
        "Principal" : {
          "AWS" : "arn:aws:iam::${var.account_id}:role/api_server_role"
        },
        "Action" : "s3:*",
        "Resource" : "arn:aws:s3:::crustchan-resources/*"
      },
      {
        "Sid" : "allow-account-access-from-account",
        "Effect" : "Allow",
        "Principal" : {
          "AWS" : "arn:aws:iam::${var.account_id}:root"
        },
        "Action" : "s3:*",
        "Resource" : "arn:aws:s3:::crustchan-resources/*"
      },
      {
        "Sid" : "DenyPublicListPutEtc",
        "Effect" : "Deny",
        "Principal" : {
          "AWS" : "*"
        },
        "Action" : [
          "s3:PutObject",
          "s3:DeleteObject"
        ],
        "Resource" : "arn:aws:s3:::crustchan-resources/*",
        "Condition" : {
          "ForAnyValue:StringNotEquals" : {
            "aws:PrincipalAccount" : "${var.account_id}"
          }
        }
      },
      {
        "Sid" : "DenyPublicBucketStuff",
        "Effect" : "Deny",
        "Principal" : {
          "AWS" : "*"
        },
        "Action" : "s3:ListBucket",
        "Resource" : "arn:aws:s3:::crustchan-resources",
        "Condition" : {
          "ForAnyValue:StringNotEquals" : {
            "aws:PrincipalAccount" : "${var.account_id}"
          }
        }
      },
      {
        "Sid" : "AllowPublicGetObject",
        "Effect" : "Allow",
        "Principal" : {
          "AWS" : "*"
        },
        "Action" : "s3:GetObject",
        "Resource" : "arn:aws:s3:::crustchan-resources/*"
      }
    ]
  })
}


resource "aws_s3_bucket_website_configuration" "app_resources" {
  bucket = aws_s3_bucket.app_resources.id
  # acl    = "public-read"
  index_document {
    suffix = "index.html"
  }

  error_document {
    key = "error.html"
  }
}

resource "aws_dynamodb_table" "crustchan_posts" {
  name         = "crustchan-posts"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"
  # range_key = "created_at"
  stream_enabled   = true
  stream_view_type = "NEW_IMAGE"
  attribute {
    name = "id"
    type = "S"
  }
  attribute {
    name = "board_id"
    type = "S"
  }
  attribute {
    name = "created_at"
    type = "S"
  }

  attribute {
    name = "op"
    type = "S"
  }
  attribute {
    name = "ip"
    type = "S"
  }


  global_secondary_index {
    name            = "board-index"
    hash_key        = "board_id"
    range_key       = "created_at"
    projection_type = "ALL"
  }
  global_secondary_index {
    name            = "ip-index"
    hash_key        = "ip"
    range_key       = "created_at"
    projection_type = "ALL"
  }
  global_secondary_index {
    name            = "OP-index"
    hash_key        = "op"
    range_key       = "created_at"
    projection_type = "ALL"
  }

  tags = {
    name        = var.name
    environment = var.environment
  }

}


resource "aws_dynamodb_table" "crustchan_boards" {
  name         = "crustchan-boards"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"
  range_key    = "created_at"
  attribute {
    name = "id"
    type = "S"
  }
  attribute {
    name = "name"
    type = "S"
  }
  attribute {
    name = "created_at"
    type = "S"
  }

  global_secondary_index {
    name            = "name-index"
    hash_key        = "name"
    range_key       = "created_at"
    projection_type = "ALL"
  }
  tags = {
    name        = var.name
    environment = var.environment
  }
}


resource "aws_dynamodb_table" "crustchan_admin" {
  name         = "crustchan-admin"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"
  range_key    = "created_at"
  attribute {
    name = "id"
    type = "S"
  }
  attribute {
    name = "username"
    type = "S"
  }
  attribute {
    name = "created_at"
    type = "S"
  }

  global_secondary_index {
    name            = "username-index"
    hash_key        = "username"
    range_key       = "created_at"
    projection_type = "ALL"
  }
  tags = {
    name        = var.name
    environment = var.environment
  }
}
module "lambda_function" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "7.14.0"

  function_name          = "crustchan-approve-post"
  handler                = "approve_post_handler"
  runtime                = "provided.al2023"
  architectures          = ["arm64"]
  create_package         = false
  local_existing_package = "../app/target/lambda/crustchan-approve-post/bootstrap.zip"
  environment_variables = {
    RUST_LOG = "crustchan-approve-post::debug"
  }
  tags = {
    name        = var.name
    environment = var.environment
  }
  ignore_source_code_hash = true
}

resource "aws_lambda_event_source_mapping" "dynamodb-stream-to-lambda" {
  event_source_arn  = aws_dynamodb_table.crustchan_posts.stream_arn
  function_name     = module.lambda_function.lambda_function_arn
  starting_position = "LATEST"
}
resource "aws_ecr_repository" "crustchan-repo" {
  name                 = "crustchan"
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }
  tags = {
    name        = var.name
    environment = var.environment
  }
}


