resource "aws_internet_gateway" "gw-lrmap" {
  vpc_id = aws_vpc.vpc-lrmap.id

  tags = {
    project = local.project
  }
}

resource "aws_route_table" "rt-lrmap-public" {
  vpc_id = aws_vpc.vpc-lrmap.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.gw-lrmap.id
  }


  tags = {
    project = local.project
  }
}

resource "aws_route_table" "rt-lrmap-private" {
  vpc_id = aws_vpc.vpc-lrmap.id


  tags = {
    project = local.project
  }
}

resource "aws_route_table_association" "rt-public-ec2" {
  route_table_id = aws_route_table.rt-lrmap-public.id
  subnet_id      = aws_subnet.subnet-lrmap-ec2.id
}

resource "aws_route_table_association" "rt-private-redis" {
  route_table_id = aws_route_table.rt-lrmap-private.id
  subnet_id      = aws_subnet.subnet-lrmap-elasticcache.id
}

resource "aws_route_table_association" "rt-private-redis-2" {
  route_table_id = aws_route_table.rt-lrmap-private.id
  subnet_id      = aws_subnet.subnet-lrmap-elasticcache-2.id
}
