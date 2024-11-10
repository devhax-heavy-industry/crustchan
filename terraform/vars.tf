

variable "environment" {
  description = "The environment."
  default     = "dev"
}

variable "name" {
  description = "The name of the application."
  default     = "crustchan"
}

variable "AWS_DEFAULT_REGION" {
  description = "The AWS default region"
  default     = "us-west-2"
}
variable "aws_default_az" {
  description = "The AWS default availability zone"
  default     = "us-west-2a"
  
}
variable "AWS_ACCESS_KEY_ID" {
  description = "value of AWS_ACCESS_KEY_ID"
  default =""
}
variable "AWS_SECRET_ACCESS_KEY" {
  description = "value of AWS_SECRET"
  default =""
}