use std::{
    io::Read,
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
};

use byteorder::WriteBytesExt;

use crate::{
    enums::{map_size, network_communication::NetworkCommunication},
    room_settings::RoomSettings,
};

pub mod enums;
pub mod read_stream;
pub mod room_settings;

fn handle_client(addr: SocketAddr, mut stream: TcpStream) -> anyhow::Result<()> {
    let buf: &mut Vec<u8> = &mut vec![];
    if let Ok(buf_size) = stream.read_to_end(buf) {
        let mut stream_buf = read_stream::Stream::new(buf.to_vec());

        let conn_type = match stream_buf.read() {
            Some(conn) => conn,
            None => anyhow::bail!("Unable to read the package type"),
        };

        let conn_type: enums::network_communication::NetworkCommunication =
            match num::FromPrimitive::from_u8(conn_type) {
                Some(conn_type) => conn_type,
                None => anyhow::bail!("Invalid package type"),
            };
        println!("conn_type: {:#?}", conn_type);

        let port = stream_buf.read_u16();
        println!("port: {:#?}", port);
        let password_protected = match stream_buf.read_bool() {
            Some(pass) => pass,
            None => anyhow::bail!("Unable to read the Port"),
        };
        println!("password_protected: {}", password_protected);

        println!("server_name: {}", stream_buf.read_str());
        println!("gamemode_name: {}", stream_buf.read_str());
        println!("map_name: {}", stream_buf.read_str());

        let map_size = match stream_buf.read() {
            Some(map_size) => map_size,
            None => anyhow::bail!("Unable to read the MapSize"),
        };
        let map_size: enums::map_size::MapSize = match num::FromPrimitive::from_u8(map_size) {
            Some(map_size) => map_size,
            None => anyhow::bail!("Invalid map size"),
        };
        println!("map_size: {:#?}", map_size);

        let map_day_night = match stream_buf.read() {
            Some(map_day_night) => map_day_night,
            None => anyhow::bail!("Unable to read the MapDayNight"),
        };
        let map_day_night: enums::map_day_night::MapDayNight =
            match num::FromPrimitive::from_u8(map_day_night) {
                Some(map_day_night) => map_day_night,
                None => anyhow::bail!("Invalid map daynight"),
            };
        println!("day_night: {:#?}", map_day_night);

        let current_players = match stream_buf.read() {
            Some(players) => players,
            None => anyhow::bail!("Unable to read the Current Players"),
        };
        println!("current_players: {}", current_players);

        let queue_players = match stream_buf.read() {
            Some(queue) => queue,
            None => anyhow::bail!("Unable to read the Queue Players"),
        };
        println!("queue_players: {}", queue_players);

        let max_players = match stream_buf.read() {
            Some(max) => max,
            None => anyhow::bail!("Unable to read the Max Players"),
        };
        println!("max_players: {}", max_players);

        println!("loading_screen_text: {}", stream_buf.read_str());
        println!("server_rules_text: {}", stream_buf.read_str());

        let ip_bytes = match addr.ip() {
            IpAddr::V4(ip) => ip.octets().to_vec(),
            IpAddr::V6(ip) => ip.octets().to_vec(),
        };
        #[allow(arithmetic_overflow)]
        let hash = (port as u32) << 32 | u32::from_le_bytes(ip_bytes.try_into().unwrap());
        println!("{:#?}", hash);

        let room_size = stream_buf.read_u32();
        let mut room_settings = RoomSettings::new();
        room_settings.read(read_stream::Stream::new(stream_buf.next_item(room_size)));
        println!("room_settings: {:#?}", room_settings);

        // let mut count = stream_buf.read_u32();
        // println!("rotation_size: {}", count);

        // while count > 0 {
        //     count -= 1;
        // }

        // stream_buf.debug();

        let _ = stream.write_u8(NetworkCommunication::Accepted as u8);
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .start()
        .unwrap();
    log::info!("Startup...");
    let listener = TcpListener::bind("0.0.0.0:29294")?;

    // accept connections and process them serially
    while let Ok((stream, addr)) = listener.accept() {
        handle_client(addr, stream);
    }
    Ok(())
}
