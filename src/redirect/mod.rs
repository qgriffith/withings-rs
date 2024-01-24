use tiny_http::{Server, Response};
use url::Url;
use std::collections::HashMap;

// Set up a server to listen for the OAuth2 redirect and returns the code and state from the redirect URL as a HashMap. 
//It binds to localhost on port 8888.
pub mod server {
    use super::*;

    pub fn run() ->  HashMap<&'static str, String>  {
        
        //Create Tiny-Http server
        let server = Server::http("0.0.0.0:8888").unwrap();
        println!("Listening on port 8888 for redirect of OAuth2 code.");
    
        let mut code = String::new();
        let mut state = String::new();
        let mut params = HashMap::new();


        //Listen for redirect
        for req in server.incoming_requests() {
            
            //Get the URL from the request and format it with the query parameters. 
            //Tiny-Http doesn't parse the URL, so we have to do it ourselves.
            let url = format!("http://localhost{}", req.url());
            let parsed_url = Url::parse(&url).unwrap();
            
            //Get the code and state from the query parameters
            code = parsed_url.query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, value)| value.into_owned())
                .unwrap_or_default();
            
            //Get the state from the query parameters
            state = parsed_url.query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, value)| value.into_owned())
                .unwrap_or_default();
            
            //Insert the code and state into a HashMap
            params.insert("code", code.to_string());
            params.insert("state", state.to_string());

            //Respond to the request
            let response = Response::from_string("Please return to the terminal.");
            req.respond(response).unwrap();

            //Stop the server
            break;
        }
        
        //Return the HashMap
        params
    }
}