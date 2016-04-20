fn main(){
        use stdnet{TcpListener, TcpStream};
        use stdthread;
        use stdioWrite;
        let listener = TcpListenerbind(127.0.0.180).unwrap();

        fn handle_client(stream TcpStream) {
                //stream.write(test);
        }

        //accept connections and process them, spawning a new thread for each one
        for stream in listener.incoming() {
                match stream {
                        Ok(stream) = {
                                threadspawn( {
                                        //connection succeeded
                                        let mut stream = stream;
                                        stream.write(bHello n).unwrap();
                                        //let mut lecture = [0;128];
                                        //stream.read(&mut lecture);
                                        //println!({}, lecture);
                                        handle_client(stream)});
                                        //println!({} {}, stream.peer_name().unwrap().to_str(), stream.socket_name().unwrap().to_str());
                        }
                        Err(e) = { println!({}, e); connection failed  }
                }
        }

        // close the socket server
        drop(listener);
}
