# HELAPI
This is a dead simple script that can be used with [Helix Editor](https://github.com/helix-editor/helix).
Its main purpose is to ease the workflow of testing APIs.

# Usage
To use it, simple select the request you want to send, pipe it using `|`, followed by the 
command `helapi`. The response of the api will show underneath the request on your editor.
Yes, just like those HTTP request function on editors like VSCode and JetBrains suite.

If you did not select anything when pipe, it will return some sample templates for you to use.

Currently it only support sending one api request per time.

Only support GET and POST now.  
Only support configuring the POST body (in JSON) now. Note that even if you are 
sending other format like `x-www-form-urlencoded`, you still need to use json format 
to config, and specify your content type in `content_type`.  
No headers support now.  

# Install
Simply clone it, cd into it, and use your cargo to build it and put it into the PATH.

# Format
The format of your request should be JSON, like below:

```json
{
	"target": "http://localhost:5555/books",
	"method": "post",
	"content_type": "application/json",
	"body": 
		{
		    "title": "ndss",
			"author": "adsfsadf",
			"publishYear": "1234"
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
```
