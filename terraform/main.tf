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
      version = "~> 5.70.0"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  access_key = var.AWS_ACCESS_KEY_ID
  secret_key = var.AWS_SECRET_ACCESS_KEY
  region     = var.AWS_DEFAULT_REGION
}