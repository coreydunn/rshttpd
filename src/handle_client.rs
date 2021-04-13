pub fn handle_client(client_sock:&mut std::net::TcpStream,addr:std::net::SocketAddr)
{
    print!("Connected to client at {:?}\n",addr);
    //let mut v:Vec<u8>=vec!();

    // Handle HTTP requests
    'connection: loop
    {
        let mut request:String="".to_string();
        let mut sock_buffer:[u8;10]=[0,0,0,0,0,0,0,0,0,0];

        // Read single request
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

            // Apply security measures here
            if uri_s.len()<1
            {
                continue 'connection;
            }

            if !safe_uri(uri_s.join("/").as_str())
            {
                print!("404 Not Found: {:?}\t=>\t[peer: {}]\n",uri.as_str(),addr);
                if !send_404(client_sock,addr)
                {
                    break 'connection;
                }
                continue 'connection;
            }

            uri=uri_s.join("/");
        }

        if uri.trim() == ""
        {
            continue 'connection;
        }

        print!("GET: {:?}\t<=\t[peer: {}]\n",uri.as_str(),addr);

        // GET URI AND SEND
        let mut uri_data="".to_string();
        match std::fs::File::open(uri.as_str())
        {
            Ok(mut f) => {
                std::io::Read::read_to_string(&mut f,&mut uri_data).unwrap();

                let response=format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",uri_data.len(),uri_data);
                if ! write_sock(client_sock,response.as_str())
                {
                    print!("Disconnected from client ({:?})\n",addr);
                    break 'connection;
                }
                print!("200 OK: {:?}\t=>\t[peer: {}]\n",uri.as_str(),addr);
            },

            // Failed to load URI -- present 404 page
            Err(_) => {
                print!("404 Not Found: {:?}\t=>\t[peer: {}]\n",uri.as_str(),addr);
                if !send_404(client_sock,addr)
                {
                    break 'connection;
                }
            },
        };
    }

    let msg=format!("error: failed to shutdown [peer: {}]\n",addr);
    match client_sock.shutdown(std::net::Shutdown::Both)
    {
        Ok(_) => print!("successfully disconnected\n"),
        Err(_) => {
            print!("{}",msg)
        }
    }
}

fn write_sock(client_sock:&mut std::net::TcpStream,response:&str) -> bool
{
    match std::io::Write::write(client_sock,
                                response
                                .as_bytes())
    {
        Ok(__) => true,
        Err(__) => {
            //print!("Disconnected from client ({:?})\n",addr);
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

pub fn safe_uri(path:&str) -> bool
{
    // Disallow files starting with "."
    // NOTE: this wille ensure file is not
    // a dotfile and is not in a higher directory
    if path.is_empty()
    {
        return false;
    }
    if path.chars().nth(0).unwrap()=='.'
    {
        return false;
    }

    return true;
}

// Return false if message failed to send
fn send_404(client_sock:&mut std::net::TcpStream,addr:std::net::SocketAddr) -> bool
{
    let mut uri_data="".to_string();

    match std::fs::File::open("404.html")
    {
        // Display /404.html page
        Ok(mut file) => {
            std::io::Read::read_to_string(&mut file,&mut uri_data).unwrap();

            let response=format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",uri_data.len(),uri_data);
            if ! write_sock(client_sock,response.as_str())
            {
                print!("Disconnected from client ({:?})\n",addr);
                //break 'connection;
                return false;
            }
        },

        // Otherwise display minimal built-in 404 message
        Err(_) => {
            uri_data="<html>404 Error: Not Found</html>".to_string();
            let response=format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",uri_data.len(),uri_data);

            if ! write_sock(client_sock,response.as_str())
            {
                print!("Disconnected from client ({:?})\n",addr);
                //break 'connection;
                return false;
            }
        },
    }

    return true;
}
