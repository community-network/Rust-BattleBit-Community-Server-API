use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use byteorder::WriteBytesExt;

use crate::{enums::network_communication::NetworkCommunication, room_settings::RoomSettings};

pub mod enums;
pub mod read_stream;
pub mod room_settings;

fn handle_client(mut stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    let buf: &mut Vec<u8> = &mut vec![];
    if let Ok(buf_size) = stream.read_to_end(buf) {
        let mut stream_buf = read_stream::Stream::new(buf.to_vec());

        let conn_type: enums::network_communication::NetworkCommunication =
            num::FromPrimitive::from_u8(stream_buf.read()).unwrap();
        println!("conn_type: {:#?}", conn_type);
        let port = stream_buf.read_u16();
        println!("port: {:#?}", port);
        println!("password_protected: {}", stream_buf.read_bool());
        println!("server_name: {}", stream_buf.read_str());
        println!("gamemode_name: {}", stream_buf.read_str());
        println!("map_name: {}", stream_buf.read_str());

        let map_size: enums::map_size::MapSize =
            num::FromPrimitive::from_u8(stream_buf.read()).unwrap();
        println!("map_size: {:#?}", map_size);

        let map_day_night: enums::map_day_night::MapDayNight =
            num::FromPrimitive::from_u8(stream_buf.read()).unwrap();
        println!("day_night: {:#?}", map_day_night);

        println!("current_players: {}", stream_buf.read());
        println!("queue_players: {}", stream_buf.read());
        println!("max_players: {}", stream_buf.read());

        println!("loading_screen_text: {}", stream_buf.read_str());
        println!("server_rules_text: {}", stream_buf.read_str());

        // let ip_bytes = match addr.ip() {
        //     IpAddr::V4(ip) => ip.octets().to_vec(),
        //     IpAddr::V6(ip) => ip.octets().to_vec(),
        // };

        let room_size = stream_buf.read_u32();
        let mut room_settings = RoomSettings::new();
        room_settings.read(read_stream::Stream::new(stream_buf.next_item(room_size)));
        println!("room_settings: {:#?}", room_settings);

        let mut count = stream_buf.read_u32();
        println!("rotation_size: {}", count);

        // while count > 0 {
        //     count -= 1;
        // }

        stream_buf.debug();

        // let _ = stream.write_u8(NetworkCommunication::Accepted as u8);
    }
}

fn main() -> std::io::Result<()> {
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .start()
        .unwrap();
    log::info!("Startup...");
    let listener = TcpListener::bind("0.0.0.0:29294")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                log::error!("Failed to connect to server: {:#?}", e)
            }
        }
    }
    Ok(())
}
