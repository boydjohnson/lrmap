data "aws_ami" "ubuntu" {
  most_recent = true

  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-arm64-server-*"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }

  owners = ["099720109477"] # Canonical
}

resource "aws_instance" "lrmap-map" {
  ami           = data.aws_ami.ubuntu.id
  instance_type = "t4g.nano"
  subnet_id     = aws_subnet.subnet-lrmap-ec2.id



  tags = {
    Name = local.project
  }
}
