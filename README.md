rust-jwt
===

[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

## Motivation
As a developer of web-based service landscapes, I often stumbled over JWT. Mostly because I wanted to test my configuration for keycloak-driven application security frameworks.  
Because of that, I often used websites like jwt.io or base64decode.org. But because I don't like to forge arbitrary traffic and believe that "the web" and "web technologies" are a big mistake, I really wanted a solution to debug my JWTs in a fashion where humans(Developers) can read used tokens. 
And may it be just to make sure, their attribute mappings are correct.

So there is Rust. A language which I never knew before and which I wanted to use in this particular case. 
Why? Because I thought it is most fitting for a small, simple command line program.

Here I present `jwt`. A simple command line tool which takes an base64Url encoded JWT and ends up with the decoded header and body of it.
Just to make sure, others do not have to use these pesky websites.

## Description
`jwt` is very simple. It takes a base64Url encoded String and decodes its information. 
As this tool does not make use of the JWTs signature, it is omitted entirely. That behaviour might change in the future.

## Install
As of now, this tool is solely installable by downloading this code and issuing `cargo` in the root of the project:  
```
cargo install --path .
```

Binary releases will be coming soon™

## Usage
```
jwt 0.3.0

jwt displays a given base64 encoded JWT String in a readable manner.
JWT header and body are printed in separate lines.

Syntax: jwt [Options] <JWT>

Options:
	--help,  -h	 View this help message
```

### Recommendation
If you happen to like pretty-printed JSON, I suggest piping the output of `jwt` through a filter like `jq`, like this:  
```
jwt eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c | jq
```

## License
This project is licensed under MIT.