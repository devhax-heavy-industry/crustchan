# Crustchan

### Running

When you first run crustchan via `cargo run` you'll notice in the logs that an admin user has been created for you. The credentials are below:

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
This cookie will be checked on all admin/sensitive routes.
