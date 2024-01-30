use tiny_http::{Server, Response};
use url::Url;
use std::collections::HashMap;
use log::{info, warn, trace};
use std::process;

//Set up a server to listen for the OAuth2 redirect and returns the code and state from the redirect URL as a HashMap. 
//It binds to localhost on port 8888.
//The server is stopped after the redirect is received and the code and state are returned.
//If the code or state are not received, the program will exit.

pub mod server {
    use super::*;
    #[allow(unused_assignments)]
    pub fn run() ->  HashMap<&'static str, String>  {
        
        //Create Tiny-Http server
        let server = Server::http("0.0.0.0:8888").unwrap_or_else(|e| {
            warn!("Could not bind to port 8888: {}", e);
            panic!("Could not bind to port 8888: {}", e);
        });

        info!("Listening on port 8888 for redirect of OAuth2 code.");
    
        let mut code = String::new();
        let mut state = String::new();
        let mut params = HashMap::new();


        //Listen for redirect
        let req = server.incoming_requests().next();
        if let Some(req) = req {
            
            //Get the URL from the request and format it with the query parameters. 
            //Tiny-Http doesn't parse the URL, so we have to do it ourselves.
            let url = format!("http://localhost{}", req.url());
            let parsed_url = Url::parse(&url).unwrap();
            
            //Get the code and state from the query parameters
            code = parsed_url.query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, value)| value.into_owned())
                .unwrap_or_default();
            
            trace!("Code: {}", code);
            
            //Get the state from the query parameters
            state = parsed_url.query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, value)| value.into_owned())
                .unwrap_or_default();
            
            //Insert the code and state into a HashMap
            if state.is_empty() || {code.is_empty()} {
                warn!("Could not get code or state from redirect URL. Exiting.");
                process::exit(1);
            }

            trace!("State: {}", state);

            params.insert("code", code.to_string());
            params.insert("state", state.to_string());

            //Respond to the request
            let response = Response::from_string("Please return to the terminal.");
            req.respond(response).unwrap_or_else(|e| {
                warn!("Could not respond to request: {}", e);
                panic!("Could not respond to request: {}", e);
            });

            //Stop the server
            drop(server);
        }
        
        //Return the HashMap
        params
    }
}