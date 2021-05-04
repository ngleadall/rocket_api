# Rocket API

Testing out a Rocket + Diesel/SQLite json HTTP API. 

**User model**

{ id: i32 , name: string, email: string , created_at: string }

**RESTful routes:**
- [GET]/users 

- [GET]/users/<id> - 

- [POST]/users

- [PUT]/users/<id>

- [DELETE]/users/<id>


**NOTE:** 

Basic auth is implemented (https://en.wikipedia.org/wiki/Basic_access_authentication).
Requests must have the below header in addition to application json 

Authorization: Basic dGVzdF91c2VyOnRlc3RfdG9rZW4=

(test_user:test_token)

**Comments**

04/21 - Rocket seems to be undergoing alot of undocumented dev atm. Will move to Actix in future as seems more stable
