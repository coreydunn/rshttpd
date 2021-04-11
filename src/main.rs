mod handle_client;
fn main()
{
    let hostname:&str;
    let argv:Vec<String>=std::env::args().collect();

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

    let s=match std::net::TcpListener::bind(format!("{}{}",hostname,":80"))
    {
        Ok(ss) => ss,
        Err(_) => {
            print!("error: failed to bind to IP address {}\n",hostname);
            std::process::exit(1);
        },
    };

    let mut threads:std::vec::Vec<std::thread::JoinHandle<()>>=vec!();
    loop
    {
        threads.push(match s.accept()
        {
            Ok((mut sock,addr)) =>std::thread::spawn(move||{handle_client::handle_client(&mut sock,addr)}),
            Err(_e) => {
                print!("error: failed to get client\n");
                std::thread::spawn(||{/*nothing*/})
            },
        })
    }
}