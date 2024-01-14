resource "aws_vpc" "vpc-lrmap" {
  cidr_block = "10.232.25.0/24"

  tags = {
    project = local.project
  }
}

resource "aws_subnet" "subnet-lrmap-ec2" {
  vpc_id                  = aws_vpc.vpc-lrmap.id
  cidr_block              = "10.232.25.0/28"
  map_public_ip_on_launch = true

  tags = {
    project = local.project
  }
}

resource "aws_subnet" "subnet-lrmap-elasticcache" {
  vpc_id                  = aws_vpc.vpc-lrmap.id
  cidr_block              = "10.232.25.16/28"
  map_public_ip_on_launch = false

  tags = {
    project = local.project
  }
}

resource "aws_subnet" "subnet-lrmap-elasticcache-2" {
  vpc_id                  = aws_vpc.vpc-lrmap.id
  cidr_block              = "10.232.25.32/28"
  map_public_ip_on_launch = false
  availability_zone       = "us-east-1d"


  tags = {
    project = local.project
  }
}

resource "aws_elasticache_subnet_group" "subnet-group-lrmap-redis" {
  name        = "elcache-lrmap-subnet-group"
  subnet_ids  = [aws_subnet.subnet-lrmap-elasticcache.id, aws_subnet.subnet-lrmap-elasticcache-2.id]
  description = "subnet group for ElasticCache Redis"
}
