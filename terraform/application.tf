resource "aws_instance" "ec2" {

  ami                         = "ami-001651dd1b19ebcb6" # ubuntu 24.04
  instance_type               = "t2.micro"

  subnet_id                   = aws_subnet.public_subnet.id
  vpc_security_group_ids      = [aws_security_group.ec2_security_group.id]
  associate_public_ip_address = true

  key_name                    = aws_key_pair.ec2_key_pair.key_name

  tags = {
    environment = var.environment
  }
}


resource "aws_security_group" "ec2_security_group" {
  name        = "cc-security-group"
  description = "security group for ec2 instances"

  vpc_id      = aws_vpc.vpc.id

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

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    environment = var.environment
  }
}

resource "aws_key_pair" "ec2_key_pair" {
  key_name   = "free-tier-ec2-key"
  public_key = file("ssh-key.pub")
}

resource "aws_s3_bucket" "app_resources" {
  bucket = "crustchan-resources"

  tags = {
    Name        = "Crustchan Resources"
    Environment = "Dev"
  }
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
  name           = "crustchan-posts"
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "id"
  range_key = "created_at"
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
  attribute {
    name = "session"
    type = "S"
  }
  
    global_secondary_index {
    name               = "board-index"
    hash_key           = "board_id"
    range_key = "created_at"
    projection_type    = "ALL"
  }
  global_secondary_index {
    name               = "ip-index"
    hash_key           = "ip"
    range_key = "created_at"
    projection_type    = "ALL"
  }
  global_secondary_index {
    name               = "OP-index"
    hash_key           = "op"
    range_key = "created_at"
    projection_type    = "ALL"
  }
  global_secondary_index {
    name               = "session-index"
    hash_key           = "session"
    range_key = "created_at"
    projection_type    = "ALL"
  }

  tags = {
    environment = var.environment
  }
  
}


resource "aws_dynamodb_table" "crustchan_boards" {
  name           = "crustchan-boards"
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "id"
  range_key = "created_at"
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
    name               = "name-index"
    hash_key           = "name"
    range_key = "created_at"
    projection_type    = "ALL"
  }
    tags = {
    environment = var.environment
  }
}


resource "aws_dynamodb_table" "crustchan_admin" {
  name           = "crustchan-admin"
  billing_mode   = "PAY_PER_REQUEST"
  hash_key       = "id"
  range_key = "created_at"
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
    name               = "username-index"
    hash_key           = "username"
    range_key = "created_at"
    projection_type    = "ALL"
  }
    tags = {
    environment = var.environment
  }
}