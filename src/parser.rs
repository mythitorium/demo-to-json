// Code taken from https://github.com/demostf/inspector
// Minor modifications were made 

use std::fs;

use bitbuffer::{BitRead, BitReadBuffer, BitReadStream, LittleEndian};
use serde::Serialize;
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::packet::datatable::{DataTablePacket, SendTableName, ServerClassName};
use tf_demo_parser::demo::packet::Packet;
use tf_demo_parser::demo::parser::DemoHandler;
use tf_demo_parser::demo::parser::RawPacketStream;
use tf_demo_parser::demo::sendprop::SendPropName;

#[derive(Debug, Serialize)]
pub struct PacketMeta {
    index: usize,
    tick: u32,
    ty: u8,
}


#[derive(Debug, Serialize)]
pub struct Parser {
    header: Header,
    packets: Vec<Packet<'static>>,
    prop_names: Vec<(u64, SendTableName, SendPropName)>,
    class_names: Vec<(u16, ServerClassName)>,
}

impl Parser {
    pub fn new(path: &str) -> Self {
        let input = fs::read(path).unwrap_or(Vec::new());

        let buffer = BitReadBuffer::new_owned(input, LittleEndian);
        let mut stream = BitReadStream::new(buffer);
        let header = Header::read(&mut stream).unwrap();

        let mut packet_stream = RawPacketStream::new(stream);
        let mut handler = DemoHandler::default();
        handler.handle_header(&header);

        let mut packets = Vec::new();

        let mut prop_names = Vec::new();
        let mut class_names = Vec::new();

        while let Some(packet) = packet_stream.next(handler.get_parser_state()).unwrap() {
            let tick = u32::from(packet.tick());
            packets.push(packet.clone());

            if let Packet::DataTables(DataTablePacket {
                tables,
                server_classes,
                ..
            }) = &packet
            {
                for table in tables {
                    for prop in &table.props {
                        prop_names.push((
                            prop.identifier.into(),
                            prop.identifier
                                .table_name()
                                .unwrap_or_else(|| table.name.clone()),
                            prop.identifier
                                .prop_name()
                                .unwrap_or_else(|| prop.name.clone()),
                        ));
                    }
                }
                for class in server_classes {
                    class_names.push((class.id.into(), class.name.clone()))
                }
            }

            handler.handle_packet(packet).unwrap();
        }

        Parser {
            header,
            packets,
            prop_names,
            class_names,
        }
    }
}