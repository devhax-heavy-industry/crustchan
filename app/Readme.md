# Crustchan

### Infrastructure

This project makes use of Amazon's free tier services so that it may be ran 100% on the free tier. The infrastructure can be setup with terraform. If you fork this project, you might want to integrate it with the app at terraform.io to build and deploy the terraform plans on every merge to `main`.

### Building the Lambda

The post approval lambda will need to be built with `cargo run-script build-lambda` It creates a zip file that can be uploaded as the lambda

### Running the Api Server

You will need to have the following environment variables set before running the crustchan api server.

```
RUST_LOG=crustchan-api=info
AWS_ACCESS_KEY_ID=xyz
AWS_SECRET_ACCESS_KEY=aaa

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
