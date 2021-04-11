pub fn handle_client(c:&mut std::net::TcpStream,addr:std::net::SocketAddr)
{
    print!("Connected to client at {:?}\n",addr);
    //let mut v:Vec<u8>=vec!();
    let mut req:String="".to_string();
    let mut h:[u8;10]=[0,0,0,0,0,0,0,0,0,0];

    'read_output: while std::io::Read::read(c,&mut h).unwrap() > 0
    {
        let st=std::str::from_utf8(&h).expect("Found invalid UTF-8 string");

        req.push_str(st);
        if req.contains("\r\n\r\n")
        {
            //let m:Vec<&str>=st.split("\n\n").collect();
            break 'read_output;
        }
    }

    // Get URI
    let mut uri:&str="";
    if req.len() > 0
    {
        if &req[0..3]=="GET"
        {
            uri=std::iter::Iterator
                ::nth(&mut req.split(" "),1).unwrap();
        }
    }

    // Validate URI
    // Index
    if uri=="/"
    {
        uri="index.html";
    }

    // Other page/URI
    else
    {
        uri=std::iter::Iterator::nth(
            &mut
            std::iter::Iterator
            ::nth(&mut req.split("/"),1)
            .unwrap().split(" "),0).unwrap();
    }

    print!("GET: {:?}\n",uri);

    let mut uri_data="".to_string();
    match std::fs::File::open(uri)
    {
        Ok(mut f) => {
            std::io::Read::read_to_string(&mut f,&mut uri_data).unwrap();

            let response=format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",uri_data.len(),uri_data);
            std::io::Write::write(c,
                                  response
                                  .as_bytes()).unwrap();
        },

        // Failed to load URI -- present 404 page
        Err(_) => {
            match std::fs::File::open("404.html")
            {
                // Display /404.html page
                Ok(mut ff) => {
                    std::io::Read::read_to_string(&mut ff,&mut uri_data).unwrap();

                    let response=format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",uri_data.len(),uri_data);
                    std::io::Write::write(c,
                                          response
                                          .as_bytes()).unwrap();
                },

                // Otherwise display minimal built-in 404 message
                Err(_) => {
                    uri_data="<html>404 Error: Not Found</html>".to_string();
                    let response=format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",uri_data.len(),uri_data);
                    std::io::Write::write(c,
                                          response
                                          .as_bytes()).unwrap();
                },
            }
        },

    };

}
