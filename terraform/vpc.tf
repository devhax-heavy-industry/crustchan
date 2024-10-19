resource "aws_vpc" "vpc" {
  cidr_block = "10.0.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    environment = var.environment
  }
}
resource "aws_route_table" "route_table" {
  count  = 1
  vpc_id = aws_vpc.vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.internet_gateway.id
  }

  tags = {
    environment = var.environment
  }
}

resource "aws_route_table_association" route_table_association {
  subnet_id      = aws_subnet.public_subnet.id
  route_table_id = concat(aws_route_table.route_table.*.id, [""])[0]
}

resource "aws_internet_gateway" "internet_gateway" {
  vpc_id = aws_vpc.vpc.id

  tags = {
    environment = var.environment
  }
}

resource "aws_subnet" "public_subnet" {
  vpc_id                  = aws_vpc.vpc.id
  cidr_block              = "10.0.1.0/24"
  availability_zone       = var.aws_default_az
  map_public_ip_on_launch = true

  tags = {
    environment = var.environment
  }
}