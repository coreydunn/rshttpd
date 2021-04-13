mod handle_client;
fn main()
{
    let hostname:&str;
    let argv:Vec<String>=std::env::args().collect();
    let mut root:String="/srv/http".to_string();

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
        let rcl=root.clone();
        threads.push(match server.accept()
        {
            Ok((mut sock,addr)) =>std::thread::spawn(move||{handle_client::handle_client(rcl,&mut sock,addr)}),
            Err(_e) => {
                print!("error: failed to get client\n");
                std::thread::spawn(||{/*nothing*/})
            },
        })
    }
}
