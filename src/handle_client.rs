pub fn handle_client(client_sock:&mut std::net::TcpStream,addr:std::net::SocketAddr)
{
    print!("Connected to client at {:?}\n",addr);
    //let mut v:Vec<u8>=vec!();

    'connection: loop
    {
        let mut request:String="".to_string();
        let mut sock_buffer:[u8;10]=[0,0,0,0,0,0,0,0,0,0];

        'read_output: while std::io::Read::read(client_sock,&mut sock_buffer).unwrap() > 0
        {
            let sock_str_buffer=std::str::from_utf8(&sock_buffer).expect("Found invalid UTF-8 string");

            request.push_str(sock_str_buffer);
            if request.contains("\r\n\r\n")
            {
                //let m:Vec<&str>=sock_str_buffer.split("\n\n").collect();
                break 'read_output;
            }

            // Ignore empty requests
            if request.trim() == ""
            {
                continue 'connection;
            }
        }

        // Get URI
        let mut uri:String="".to_string();
        if request.len() > 0
        {

            match &request[0..3]
            {
                //if &request[0..3]=="GET"
                "GET" =>
                {
                    uri=std::iter::Iterator
                        ::nth(&mut request.split(" "),1).unwrap().to_string();
                    if uri.trim() == ""
                    {
                        uri="".to_string();
                        continue 'connection;
                    }
                },

                _ =>
                {
                    // Ignore unknown requests
                    continue 'connection;
                }
            }
        }

        // Validate URI
        // Index
        if uri=="/".to_string()
        {
            uri="index.html".to_string();
        }

        // Other page/URI
        else
        {
            let uri_s=split_uri(uri.as_str());

            uri=uri_s.join("/");
        }

        if uri.trim() == ""
        {
            continue 'connection;
        }

        print!("GET: {:?}\t<=\t[peer: {}]\n",uri.as_str(),addr);
        //print!("GET: {:?}\t[peer: {}]\n",uri,addr);

        let mut uri_data="".to_string();
        match std::fs::File::open(uri.as_str())
        {
            Ok(mut f) => {
                std::io::Read::read_to_string(&mut f,&mut uri_data).unwrap();

                let response=format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",uri_data.len(),uri_data);
                if ! write_sock(client_sock,&addr,response.as_str())
                {
                    print!("Disconnected from client ({:?})\n",addr);
                    break 'connection;
                }
                print!("200 OK: {:?}\t=>\t[peer: {}]\n",uri.as_str(),addr);
            },

            // Failed to load URI -- present 404 page
            Err(_) => {

                print!("404 Not Found: {:?}\t=>\t[peer: {}]\n",uri.as_str(),addr);

                match std::fs::File::open("404.html")
                {
                    // Display /404.html page
                    Ok(mut ff) => {
                        std::io::Read::read_to_string(&mut ff,&mut uri_data).unwrap();

                        let response=format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",uri_data.len(),uri_data);
                        if ! write_sock(client_sock,&addr,response.as_str())
                        {
                            print!("Disconnected from client ({:?})\n",addr);
                            break 'connection;
                        }
                    },

                    // Otherwise display minimal built-in 404 message
                    Err(_) => {
                        uri_data="<html>404 Error: Not Found</html>".to_string();
                        let response=format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",uri_data.len(),uri_data);

                        if ! write_sock(client_sock,&addr,response.as_str())
                        {
                            print!("Disconnected from client ({:?})\n",addr);
                            break 'connection;
                        }
                    },
                }
            },

        };
    }

    print!("successfully disconnected\n");
    let msg=format!("error: failed to shutdown [peer: {}]\n",addr);
    match client_sock.shutdown(std::net::Shutdown::Both)
    {
        Ok(_) => (),
        Err(_) => {
            print!("{}",msg)
        }
    }
}

fn write_sock(client_sock:&mut std::net::TcpStream,addr:&std::net::SocketAddr,response:&str) -> bool
{
    match std::io::Write::write(client_sock,
                                response
                                .as_bytes())
    {
        Ok(__) => true,
        Err(__) => {
            print!("Disconnected from client ({:?})\n",addr);
            false
        }
    }
}

fn split_uri(u:&str) -> Vec::<&str>
{
    let mut v:Vec::<&str>=vec!();

    for x in u.split("/")
    {
        if x != ""
        {
            v.push(x.trim());
        }
    }
    v
}

fn _print_type_of<T>(_: &T)
{
    println!("Type: {}", std::any::type_name::<T>())
}
