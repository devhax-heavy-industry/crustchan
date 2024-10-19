

variable "environment" {
  description = "The environment."
  default     = "dev"
}

variable "aws_access_key" {
  description = "The AWS key"

}

variable "aws_access_secret" {
  description = "The AWS secret"

}
variable "aws_default_region" {
  description = "The AWS default region"
  default     = "us-west-2"
}
variable "aws_default_az" {
  description = "The AWS default availability zone"
  default     = "us-west-2a"
  
}