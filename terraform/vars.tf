

variable "environment" {
  description = "The environment."
  default     = "dev"
}

variable "AWS_ACCESS_KEY_ID" {
  description = "The AWS key"

}

variable "AWS_SECRET_ACCESS_KEY" {
  description = "The AWS secret"

}
variable "AWS_DEFAULT_REGION" {
  description = "The AWS default region"
  default     = "us-west-2"
}
variable "aws_default_az" {
  description = "The AWS default availability zone"
  default     = "us-west-2a"
  
}