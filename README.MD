# HELAPI
This is a dead simple script that can be used with [Helix Editor](https://github.com/helix-editor/helix).
Its main purpose is to ease the workflow of testing APIs.

To use it, simple select the request you want to send, pipe it using `|`, followed by the 
command `helapi`. The response of the api will show underneath the request on your editor.
Yes, just like those HTTP request function on editors like VSCode and JetBrains suite.

Currently it only support sending one api request per time.

Only support GET and POST now.  
Only support configuring the POST body (in JSON) now.  
No headers support now.  

# Install
Simply clone it, cd into it, and use your cargo to build it and put it into the PATH.

# Format
The format of your request should be JSON, like below:

{  
	"target": "http://localhost:5555/books",  
	"method": "post",  
	"body":  
		{  
		    "title": "ndss",  
			"author": "adsfsadf",  
			"publishYear": "1234"  
		 }  
}


