

# resource aws_lambda_function "post-post" {
#   function_name = "crustchan-post-post"
#   s3_bucket = "crustchan-lambda"
#   s3_key = "post-post.zip"
#   handler = "post-post.handler"
#   runtime = "nodejs12.x"
#   role = aws_iam_role.lambda_exec.arn
#   environment {
#     variables = {
#       ENVIRONMENT = var.environment
#     }
#   }
#   tags = {
#     environment = var.environment
#   }
# }