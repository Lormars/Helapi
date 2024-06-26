# HELAPI
This is a dead simple script that can be used with [Helix Editor](https://github.com/helix-editor/helix).
Its main purpose is to ease the workflow of testing APIs.

# Usage
To use it, simple select the request you want to send, pipe it using `|`, followed by the 
command `helapi`. The response of the api will show underneath the request on your editor.
Yes, just like those HTTP request function on editors like VSCode and JetBrains suite.


Currently it only support sending one api request per time.

Only support GET, PUT, DELETE, and POST now.  
Only support configuring the body in JSON now. Note that even if you are 
sending other format like `x-www-form-urlencoded`, you still need to use json format 
to config, and specify your content type in `content_type`.  
This tool supports GraphQL queries as shown below. Note even though the graphql queries
below are not valid json (the red part) per-se due to newline in the query string, I decide to keep it that 
way and only parse the query string to get rid of these newline characters automatically later for better
readability. So you can don't need to worry these.

Note this tool is case-sensitive. So "authorization" is not equal to "Authorization".

Lastly, for now it has only one command-line argument: `-t` flag will return some 
sample templates for you to use.

# Install
Simply clone it, cd into it, and use your cargo to build it and put it into the PATH.

# Format
The format of your request should be JSON, like below:

```json
{
	"target": "https://simple-books-api.glitch.me/orders",
	"method": "post",
	"content_type": "application/json",
	"authorization": "Bearer 2586943e89d9fee22379e16ec81470ac9a17292fa155a96d9a98be7xz7412c74",
	"headers": {
		"foo": "bar",
		"foo2": "bar2"
	},
	"body": {
		"bookId": "1",
		"clientEmail": "ndss"
	}

	
}

{
	"target": "http://localhost:3000/post/urlform",
	"content_type": "x-www-form-urlencoded",
	"method": "post",
	"body": {
		"name": "hello",
		"age": 3
	}
		
}

{
	"target": "http://localhost:4000",
	"method": "post",
	"content_type": "application/json",
	"body": 
		{
		    "query": "query Example {
				games {
					title
				}
			}"
		 }

	
}

{
	"target": "http://localhost:4000",
	"content_type": "application/json",
	"method": "post",
	"body": {
		"query": "mutation AddMutation($game: AddGameInput!) {
				addGame(game: $game) {
					id,
					platform
					}	
					
		
		}",
		"variables": {
			"game": {
				"title": "a new game",
				"platform": ["ps5", "switch"]
			}		
		}
			
		
		
	}
		
}
```
