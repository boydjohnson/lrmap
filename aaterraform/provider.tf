terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.30.0"
    }
  }
}

locals {
  project = "lrmap"
}

provider "aws" {
  default_tags {
    tags = {
      project = local.project
    }
  }
}




terraform {
  backend "s3" {
    bucket = "lrmap-terraform"
    key    = "lrmap/dev/terraform.tfstate"
    region = "us-east-1"
  }
}
