pub struct Config
{
    pub working_dir:String,
    pub log_dir:String,
}

impl Config
{
    pub fn new() -> Config
    {
        Config {
            working_dir:"/srv/http".to_string(),
            log_dir:"/var/rshttpd.log".to_string(),
        }
    }

    pub fn open(path:&str) -> Config
    {
        let mut conf=Config::new();
        let file=std::fs::File::open(path);

        match file
        {
            // Read file
            Ok(mut f) => {
                let mut working_dir:String="".to_string();
                std::io::Read::read_to_string(&mut f,&mut working_dir).unwrap();
                conf.working_dir=working_dir.trim().to_string();
                conf
            },

            Err(_) => {
                print!("warning: failed to open config file '{}', using default settings\n",
                       path);
                conf
            }
        }
    }

    pub fn print(&self)
    {
        print!("working_dir: '{}'\n",self.working_dir);
    }

    //pub fn open_log(&self)
    //{
    //}
}

pub fn handle_client_thread(conf:Config,client_sock:&mut std::net::TcpStream,addr:std::net::SocketAddr)
{
    print!("Connected to client at {:?}\n",addr);
    //let mut v:Vec<u8>=vec!();
    let thread_start_time=std::time::SystemTime::now();
    let timeout=300u64; // 5 minutes timeout

    // Get elapsed time
    fn get_elapsed_time(start_time:&std::time::SystemTime) -> u64
    {
        std::time::SystemTime::elapsed(start_time).unwrap().as_millis() as u64
    }

    // Handle HTTP requests
    'connection: loop
    {
        let mut request:String="".to_string();
        let mut sock_buffer:[u8;10]=[0,0,0,0,0,0,0,0,0,0];

        // Close after timeout
        if get_elapsed_time(&thread_start_time)>=timeout
        {
            std::net::TcpStream::shutdown(&client_sock,std::net::Shutdown::Both).unwrap();
            //print!("timeout!\n");
            print!("Timeout: \t\t[peer: {}]\n",addr);
            return;
        }

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
        //let mut uri_data="".to_string();
        let mut uri_data:Vec<u8>=vec![];
        let effective_uri=format!("{}/{}",conf.working_dir,uri.as_str());
        match std::fs::File::open(&effective_uri)
        {
            Ok(mut f) => {
                std::io::Read::read_to_end(&mut f,&mut uri_data).unwrap();

                //let response=format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",uri_data.len(),uri_data);
                let header=format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",uri_data.len());
                let mut response:Vec<u8>=vec![];

                //std::vec::Vec::append(&mut response,&mut header);
                std::vec::Vec::append(&mut response,&mut header.as_bytes().to_vec());
                std::vec::Vec::append(&mut response,&mut uri_data.to_vec());

                if ! write_sock(client_sock,&response)
                {
                    print!("Disconnected from client ({:?})\n",addr);
                    break 'connection;
                }

                print!("200 OK: {:?}\t=>\t[peer: {}]\n",effective_uri.as_str(),addr);
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

fn write_sock(client_sock:&mut std::net::TcpStream,response:&Vec<u8>) -> bool
{
    match std::io::Write::write(client_sock,response)
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
    let mut uri_data:Vec<u8>=vec![];

    match std::fs::File::open("404.html")
    {
        // Display /404.html page
        Ok(mut file) => {
            std::io::Read::read_to_end(&mut file,&mut uri_data).unwrap();

            let mut response:Vec<u8>=vec![];
            let header=format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n",uri_data.len());
            std::vec::Vec::append(&mut response,&mut header.as_bytes().to_vec());
            std::vec::Vec::append(&mut response,&mut uri_data.to_vec());

            if ! write_sock(client_sock,&response)
            {
                print!("Disconnected from client ({:?})\n",addr);
                //break 'connection;
                return false;
            }
        },

        // Otherwise display minimal built-in 404 message
        Err(_) => {
            let uri_data="<html>404 Error: Not Found</html>".as_bytes();
            let mut response:Vec<u8>=vec![];
            let header=format!("HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n",uri_data.len());
            std::vec::Vec::append(&mut response,&mut header.as_bytes().to_vec());
            std::vec::Vec::append(&mut response,&mut uri_data.to_vec());

            if ! write_sock(client_sock,&response)
            {
                print!("Disconnected from client ({:?})\n",addr);
                //break 'connection;
                return false;
            }
        },
    }

    return true;
}
