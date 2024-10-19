terraform {
  cloud {
    organization = "devhax"
    workspaces {
      name = "crustchan"
    }
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  access_key = var.aws_access_key
  secret_key = var.aws_access_secret
  region     = var.aws_default_region
}