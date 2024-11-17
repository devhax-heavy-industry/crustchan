# Crustchan

### What is it?

Crustchan is an imageboard built for the cloud in Rust, particularly to qualify for AWS's Free tier.

### Why was it built (this way)

I built this project as a way to learn AWS ECS and to refine my ability to write stuff in Rust. I was un(der) employed during this time and wanted to build something in Rust that people could use for free.

## Features

- Cloud native infrastructure
- Automated (via Amazon Rekognition) approval queue for SFW boards to keep them SFW
- Infratructure captured as code via Terraform
- - Everything is built in AWS to qualify for the **_Free Tier_**
- A CI/CD pipeline to deploy to AWS Lambda (queue moderation) and ECS (The backend microservice)

## What it is currently lacking

- A frontend.
- All the backend API endpoints implemented
- OpenAPI documentation
- Architecture diagram
- An overview of the database
- Better scaling past one instance

### Infrastructure

This project makes use of Amazon's free tier services so that it may be ran 100% on the free tier. The infrastructure can be setup with terraform. If you fork this project, you might want to integrate it with the app (Terraform HCP) at terraform.io to build and deploy the terraform plans on every merge to `main`.

### Building the Lambda

The post approval lambda will need to be built with `cargo run-script build-lambda` It creates a zip file that can be uploaded as the lambda

### Running the Api Server

You will need to have the following environment variables set before running the crustchan api server.

```
RUST_LOG=warn,crustchan-api=info #This can be changed (according to your needs)[https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html].
AWS_ACCESS_KEY_ID=xyz       # the aws user/service role should have write access to S3 and the dynamodb tables created by Terraform.
AWS_SECRET_ACCESS_KEY=aaa
S3_BUCKET=name-of-your-bucket # This bucket will hold all uploaded images
AWS_ROLE_ARN="arn:aws:iam::ACCOUNT_ID:role/ROLE_NAME" The role the service is going to assume
```

When you first run crustchan via `cargo run --bin crustchan-api` you'll notice in the logs that an admin user has been created for you. The credentials are below:

```
username: admin
password: changeme
```

Go ahead and change that user's password with the `admin/change-password` endpoint. Here's an example payload to change the password to "This is a much better password."

```
{
  "username": "admin"
  "current_password": "changeme"
  "new_password": "This is a much better password."
}
```

You can obtain an authorized session by using the `admin/login` endpoint. It sets a cookie when provided correct credentials.
This cookie will be checked on all sensitive routes.

### Support the author

Hire me: [svajlenka.com](www.svajlenka.com)
Send me some coins:

- bitcoin: 1Lg3YBZvZG2Qan7acbECdJo7Wvoh8pt7E8
- monero: 87CYcYyWp8RHRGmqVw9XHBf54NGdwYZevLfk7i47FC5FNYiGgAemJ2382gMszNgrfp4LhrFuseZTxdXqyok3S3i9S11qAix
- Ethereum: 0x4347a1AaE71f5f9DBCa602bB0A0bD856505726Bf
- Doge: DBhJp9hrMjRbP1QnvnnGXNhRsGj3re3Svi

Please send me a note [here in the discussion forum](https://github.com/devhax-heavy-industry/crustchan/discussions/2) letting me know you appreciate my software:
