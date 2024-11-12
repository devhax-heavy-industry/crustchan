resource "aws_vpc" "vpc" {
  cidr_block = "10.0.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    environment = var.environment
    name = var.name
  }
}
resource "aws_route_table" "route_table" {
  vpc_id = aws_vpc.vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.internet_gateway.id
  }

  tags = {
    environment = var.environment
    name = "${var.name}-route-table"
  }
}

resource "aws_route_table_association" route_table_association {
  subnet_id      = aws_subnet.public_subnet.id
  route_table_id = aws_route_table.route_table.id
}
resource "aws_route_table_association" "subnet2_route" {
 subnet_id      = aws_subnet.subnet2.id
 route_table_id = aws_route_table.route_table.id
}

resource "aws_internet_gateway" "internet_gateway" {
  vpc_id = aws_vpc.vpc.id

  tags = {
    environment = var.environment
    name = "${var.name}-internet-gateway"
  }
}

resource "aws_subnet" "public_subnet" {
  vpc_id                  = aws_vpc.vpc.id
  cidr_block              = "10.0.1.0/24"
  availability_zone       = var.aws_default_az
  map_public_ip_on_launch = true

  tags = {
    environment = var.environment
    name = "${var.name}-public-subnet"
  }
}
resource "aws_subnet" "subnet2" {
  vpc_id                  = aws_vpc.vpc.id
  cidr_block              = "10.0.2.0/24"
  availability_zone       = var.aws_secondary_az
  map_public_ip_on_launch = true

  tags = {
    environment = var.environment
    name = "${var.name}-public-subnet"
  }
}

# Creating EIP
resource "aws_eip" "eip" {
  domain = aws_vpc.vpc.id
}
# Creating NAT Gateway
resource "aws_nat_gateway" "gw" {
  allocation_id = aws_eip.eip.id
  subnet_id     = aws_subnet.private_subnet.id
}
# Creating Route Table for NAT Gateway
resource "aws_route_table" "rt_NAT" {
    vpc_id = aws_vpc.vpc.id
route {
        cidr_block = "0.0.0.0/0"
        nat_gateway_id = aws_nat_gateway.gw.id
    }
tags = {
        Name = "Main Route Table for Private subnets"
    }
}

resource "aws_route_table_association" "rt_associate_private" {
    subnet_id = aws_subnet.private_subnet.id
    route_table_id = aws_route_table.rt_private.id
}

resource "aws_subnet" "private_subnet" {
  vpc_id                  = aws_vpc.vpc.id
  cidr_block              = "10.0.3.0/24"
  availability_zone       = var.aws_default_az
  map_public_ip_on_launch = true

  tags = {
    environment = var.environment
    name = "${var.name}-private-subnet"
  }
}
resource "aws_subnet" "private_subnet2" {
  vpc_id                  = aws_vpc.vpc.id
  cidr_block              = "10.0.4.0/24"
  availability_zone       = var.aws_secondary_az
  map_public_ip_on_launch = true

  tags = {
    environment = var.environment
    name = "${var.name}-private-subnet2"
  }
}

# Creating Route table for Private Subnet
resource "aws_route_table" "rt_private" {
    vpc_id = aws_vpc.my_vpc.id
tags = {
        Name = "Route Table for the Private Subnet"
    }
}
resource "aws_route_table_association" "rt_associate_private_1" {
    subnet_id = aws_subnet.private_subnet.id
    route_table_id = aws_route_table.rt_private.id
}
resource "aws_route_table_association" "rt_associate_private_2" {
    subnet_id = aws_subnet.private_subnet2.id
    route_table_id = aws_route_table.rt_private.id
}