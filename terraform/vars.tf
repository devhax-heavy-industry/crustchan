

variable "environment" {
  description = "The environment."
  default     = "dev"
}
variable "account_id" {
  description = "The AWS account ID."
  default     = "611250396493"
}
variable "github_iam_role" {
  description = "The IAM role for GitHub Actions."
  default     = "GitHubActionsRole"
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
variable "aws_secondary_az" {
  description = "The AWS default availability zone"
  default     = "us-west-2b"
}
variable "AWS_ACCESS_KEY_ID" {
  description = "value of AWS_ACCESS_KEY_ID"
  default =""
}
variable "AWS_SECRET_ACCESS_KEY" {
  description = "value of AWS_SECRET"
  default =""
}