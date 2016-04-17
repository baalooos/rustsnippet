fn main(){
        use std::net::{TcpListener, TcpStream};
        use std::thread;
        use std::io::Write;
        use std::io::Read;
        let listener = TcpListener::bind("127.0.0.1:80").unwrap();

use mio::*;
use mio::net::{SockAddr};
use mio::net::tcp::{TcpSocket, TcpAcceptor};



        fn handle_client(stream: TcpStream) {
                //stream.write("test");
        }

        // accept connections and process them, spawning a new thread for each one
        for stream in listener.incoming() {
                match stream {
                        Ok(stream) => {
                            thread::spawn(move || {
                                //connection succeeded
                                //let mut stream = stream;
                                //stream.write(b"Hello \n").unwrap();
                                //let mut lecture = [0;128];
                                //stream.read(&mut lecture);
                                //println!("{}", lecture);
                                ////handle_client(stream)});
                                //println!("{} {}", stream.peer_name().unwrap().to_str(), stream.socket_name().unwrap().to_str());
                                //});
                  				let mut stream = stream;
			                	//let mut texte = true as u8;
		                    	let mut texte = String::new();
        	        			stream.write(b"hello\n\n").unwrap();
	                     		stream.read_to_string(&mut texte);

                            });
                        }
                        Err(e) => { println!("{}", e);/* connection failed */ }
                }
        }

        // close the socket server
        drop(listener);
}
