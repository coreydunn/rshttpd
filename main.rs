fn main()
{
    //let s=std::net::TcpListener::bind("127.0.0.1:80").unwrap();
    let s=std::net::TcpListener::bind("192.168.1.3:80").unwrap();

    let handle_client=|c:&mut std::net::TcpStream,addr:std::net::SocketAddr|
        {
            print!("got one ({:?})\n",addr);
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

            print!("{}",req);

            let uri="<html><body>wow</body></html>";
            let response=format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",uri.len(),uri);
            print!("SENDING RESPONSE!! {}\n",response);
            std::io::Write::write(c,
                  response
                  .as_bytes()).unwrap();
        };

    let mut threads:std::vec::Vec<std::thread::JoinHandle<()>>=vec!();
    loop
    {
        threads.push(match s.accept()
        {
            Ok((mut sock,addr)) =>std::thread::spawn(move||{handle_client(&mut sock,addr)}),
            Err(_e) => {
                print!("error: failed to get client\n");
                std::thread::spawn(||{/*nothing*/})
            },
        })
    }
}
