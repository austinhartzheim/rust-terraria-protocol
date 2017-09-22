use nom::le_u8;
use types;

// https://pastebin.com/W7vmHSNj - possibly outdated

fn do_things() {
    println!("things");
}


named!(pub pkt_header, tag!(&[15u8, 0u8]));
named!(pub pkt_subpkt(&[u8]) -> types::TerrariaPacketType,
       alt!(subpkt_disconnect | subpkt_continue_connecting));
named!(pub pkt(&[u8]) -> types::TerrariaPacketType,
       do_parse!(pkt_header >> subpkt: pkt_subpkt >> (subpkt)));

// TODO: need to handle reason string in disconnect packet.
named!(pub subpkt_disconnect(&[u8]) -> types::TerrariaPacketType,
       do_parse!(tag!(&[2u8]) >>
                 reason_length: le_u8 >>
                 reason: take_str!(reason_length) >>
                 (types::TerrariaPacketType::Disconnect(types::Disconnect {reason: reason.to_owned()}))));
named!(pub subpkt_continue_connecting(&[u8]) -> types::TerrariaPacketType,
      do_parse!(tag!(&[3u8]) >> player_id: take!(1) >>
                (types::TerrariaPacketType::ContinueConnecting(types::ContinueConnecting {player_id: player_id[0]}))));
named!(pub subpkt_player_info(&[u8]) -> types::PlayerInfo,
       do_parse!(player_id: le_u8 >>
                 skin_varient: le_u8 >>
                 hair: le_u8 >>
                 name_length: le_u8 >>
                 name: take_str!(name_length) >>
                 hair_dye: le_u8 >>
                 hide_visuals: le_u8 >>
                 hide_visuals_2: le_u8 >>
                 hide_misc: le_u8 >>
                 hair_color: color >>
                 skin_color: color >>
                 eye_color: color >>
                 shirt_color: color >>
                 under_shirt_color: color >>
                 pants_color: color >>
                 shoe_color: color >>
                 difficulty: le_u8 >> // TODO: convert to an enum
                 (types::PlayerInfo {
                     player_id, skin_varient, hair, name: name.to_owned(),
                     hair_dye, hide_visuals, hide_visuals_2, hide_misc,
                     hair_color, skin_color, eye_color, shirt_color,
                     under_shirt_color, pants_color, shoe_color, difficulty
                 })));

named!(pub color(&[u8]) -> types::Color,
       do_parse!(r: le_u8 >> g: le_u8 >> b: le_u8 >>
                 (types::Color {r: r, g: g, b: b})));


/*#[cfg(test)]
mod tests {
    use super::subpkt_player_info;
    #[test]
    fn test_player_info_deserialize_parser() {
        const PKT: [u8; 33] = [
            0x00,
            0x00,
            0x00,
            0x03,
            0x97,
            0x98,
            0x99,
            0x00,
            0x08,
            0x00,
            0x00,
            0x40,
            0x18,
            0x0d,
            0xff,
            0xbe,
            0xac,
            0xf2,
            0x8b,
            0x00,
            0x26,
            0x26,
            0x26,
            0x17,
            0x22,
            0x36,
            0x26,
            0x26,
            0x26,
            0x27,
            0x27,
            0x27,
            0x00,
        ];
        let (z, playerinfo) = subpkt_player_info(&PKT).unwrap();
        println!("z: {:?}", z);
        assert_eq!(playerinfo.name, "neo".to_owned());
    }
}
*/

// #[cfg(test)]
// mod tests {
//     use nom::IResult;
//     use types::TerrariaPacketType;
//     use parser::*;
//     use types::Color;
//     #[test]
//     fn test_pkt() {
//         let packet = [15u8, 0u8, 3u8, 5u8];
//         let result = pkt(&packet);
//         assert!(result.is_done());
//
//         let (input, output) = result.unwrap();
//         assert_eq!(input, &[]);
//         assert_eq!(
//             output,
//             TerrariaPacketType::ContinueConnecting(types::ContinueConnecting { player_id: 5 })
//         );
//     }
// }
