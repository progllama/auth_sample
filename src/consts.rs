pub const USER_DATA_PATH: &str = "data_store/users.csv";
pub const AUTH_TOKEN: &str = "AUTH_TOKEN";

pub const INDEX_PAGE: &str = r#"
<!DOCTYPE html>
<html>
        <head>
            <title>Index</title>
        </head>
    <body>
        <form action="logout", method="POST"><input type="submit" value="logout"></form>
        {message}
    </body>
</html>
"#;

pub const SIGNUP_PAGE: &str = r#"
<!DOCTYPE html>
<html>
        <head>
            <title>Signup</title>
        </head>
    <body>
        <form action="logout", method="POST"><input type="submit" value="logout"></form>
        <form action="signup" method="POST" name="SignupForm">
            <input type="text" name="name">
            <input type="email" name="email">
            <input type="password" name="password">
            <input type="submit">
        </form>
    </body>
</html>
"#;

pub const LOGIN_PAGE: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Login</title>
    </head>
    <body>
        <form action="logout", method="POST"><input type="submit" value="logout"></form>
        <form action="login" method="POST" name="LoginForm">
            <input type="email" name="email">
            <input type="password" name="password">
            <input type="submit">
        </form>
    </body>
</html>
"#;