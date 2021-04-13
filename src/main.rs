mod connection;

fn main()
{
    let argv:Vec<String>=std::env::args().collect();
    let configfile:String="/etc/rshttpd.conf".to_string();
    let hostname:&str;
    let mut conf=connection::Config::new();

    // Parse arguments
    if argv.len()<2
    {
        print!("usage: rshttpd IPADDRESS\n");
        std::process::exit(0);
    }

    else
    {
        hostname=argv[1].as_str();
    }

    // Read config file
    conf=connection::Config::open(&configfile);
    conf.print();

    // Start TCP server
    let server=match std::net::TcpListener::bind(format!("{}{}",hostname,":80"))
    {
        Ok(s) => s,
        Err(_) => {
            print!("error: failed to bind to IP address {}\n",hostname);
            std::process::exit(1);
        },
    };

    let mut threads:std::vec::Vec<std::thread::JoinHandle<()>>=vec!();
    loop
    {
        //let rcl=conf.working_dir.clone();
        threads.push(match server.accept()
        {
            Ok((mut sock,addr)) =>{
                let c:connection::Config=connection::Config{
                    working_dir:conf.working_dir.clone(),
                    log_dir:conf.log_dir.clone(),
                };
                std::thread::spawn(move||{connection::handle_client(c,&mut sock,addr)})
            },
            Err(_e) => {
                print!("error: failed to get client\n");
                std::thread::spawn(||{/*nothing*/})
            },
        })
    }
}
