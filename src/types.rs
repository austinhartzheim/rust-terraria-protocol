use nom::{le_u8, le_u64, le_i16, le_i32, le_f32};
use parser::color;

pub trait TerrariaPacket {
    fn deserialize(bytes: &[u8]) -> Self;
    fn serialize(self: &Self) -> Vec<u8> {
        vec![]
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TerrariaPacketType {
    Disconnect(Disconnect),
    ContinueConnecting(ContinueConnecting),
    PlayerInfo(PlayerInfo),
    PlayerInventorySlot(PlayerInventorySlot),
    WorldInfo(WorldInfo),
}

// --- Reused types ---

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// --- Packet types ---

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Connect {
    pub version: String,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Disconnect {
    pub reason: String,
}



impl Disconnect {}

impl TerrariaPacket for Disconnect {
    fn deserialize(bytes: &[u8]) -> Disconnect {
        named!(parse(&[u8]) -> Disconnect,
               do_parse!(reason_length: le_u8 >>
                         reason: take_str!(reason_length) >>
                         (Disconnect {reason: reason.to_owned()})));
        let (_, pkt) = parse(bytes).unwrap();
        pkt
    }

    fn serialize(self: &Self) -> Vec<u8> {
        let mut pkt = vec![self.reason.len() as u8];
        pkt.extend(self.reason.as_bytes());
        pkt
        // TODO: add header with size info
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct ContinueConnecting {
    pub player_id: u8,
}

impl TerrariaPacket for ContinueConnecting {
    fn deserialize(bytes: &[u8]) -> ContinueConnecting {
        named!(parse(&[u8]) -> ContinueConnecting,
               do_parse!(player_id: le_u8 >>
               (ContinueConnecting { player_id: player_id})));
        let (_, pkt) = parse(bytes).unwrap();
        pkt
    }

    fn serialize(self: &Self) -> Vec<u8> {
        vec![] // TODO: impelemnt
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct PlayerInfo {
    pub player_id: u8,
    pub skin_varient: u8,
    pub hair: u8,
    pub name: String,
    pub hair_dye: u8,
    pub hide_visuals: u8,
    pub hide_visuals_2: u8,
    pub hide_misc: u8,
    pub hair_color: Color,
    pub skin_color: Color,
    pub eye_color: Color,
    pub shirt_color: Color,
    pub under_shirt_color: Color,
    pub pants_color: Color,
    pub shoe_color: Color,
    pub difficulty: u8, // TODO: convert to an enum
}

impl TerrariaPacket for PlayerInfo {
    fn deserialize(bytes: &[u8]) -> PlayerInfo {
        //trace_macros!(true);
        named!(parse(&[u8]) -> PlayerInfo,
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
                         (PlayerInfo {
                             player_id, skin_varient, hair, name: name.to_owned(),
                             hair_dye, hide_visuals, hide_visuals_2, hide_misc,
                             hair_color, skin_color, eye_color, shirt_color,
                             under_shirt_color, pants_color, shoe_color, difficulty
                         })));
        //trace_macros!(false);
        let x = parse(bytes);
        if x.is_err() {
            println!("error: {}", x.unwrap_err());
        }
        let (_, pkt) = parse(bytes).unwrap();
        pkt
    }
    fn serialize(self: &Self) -> Vec<u8> {
        vec![] // TODO: implement
    }
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct PlayerInventorySlot {
    pub player_id: u8,
    pub slot_id: u8,
    pub stack: i16,
    pub prefix: u8, // TODO: should this be an enum?
    pub item_netid: i16, // TODO: should this be u16?
}

impl TerrariaPacket for PlayerInventorySlot {
    fn deserialize(bytes: &[u8]) -> PlayerInventorySlot {
        //trace_macros!(true);
        named!(parse(&[u8]) -> PlayerInventorySlot,
               do_parse!(player_id: le_u8 >>
                         slot_id: le_u8 >>
                         stack: le_i16 >>
                         prefix: le_u8 >>
                         item_netid: le_i16 >> // TODO: consider using an enum
                         (PlayerInventorySlot {
                             player_id, slot_id, stack, prefix, item_netid
                         })));
        //trace_macros!(false);
        let x = parse(bytes);
        if x.is_err() {
            println!("error: {}", x.unwrap_err());
        }
        let (_, pkt) = parse(bytes).unwrap();
        pkt
    }
    fn serialize(self: &Self) -> Vec<u8> {
        vec![] // TODO: implement
    }
}

#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
pub struct WorldInfo {
    pub time: i32, // TODO: might be worthwhile to create a time type
    pub day_moon_info: u8, // TODO: should this be an enum?
    pub moon_phase: u8,
    pub max_tiles_x: i16,
    pub max_tiles_y: i16,
    pub spawn_x: i16,
    pub spawn_y: i16,
    pub world_surface: i16,
    pub rock_layer: i16,
    pub world_id: i32,
    pub world_name: String,
    pub world_unique_id: Vec<u8>, // TODO: should this be named world_uuid?
    pub moon_type: u8,
    pub tree_background: u8,
    pub corrption_background: u8,
    pub jungle_background: u8,
    pub snow_background: u8,
    pub hallow_background: u8,
    pub crimson_background: u8,
    pub desert_background: u8,
    pub ocean_background: u8,
    pub ice_back_style: u8,
    pub jungle_back_style: u8,
    pub hell_back_style: u8,
    pub wind_speed_set: f32,
    pub cloud_number: u8,
    pub tree_1: i32,
    pub tree_2: i32,
    pub tree_3: i32,
    pub tree_style_1: u8,
    pub tree_style_2: u8,
    pub tree_style_3: u8,
    pub tree_style_4: u8,
    pub cave_back_1: i32,
    pub cave_back_2: i32,
    pub cave_back_3: i32,
    pub cave_back_style_1: u8,
    pub cave_back_style_2: u8,
    pub cave_back_style_3: u8,
    pub cave_back_style_4: u8,
    pub rain: f32,
    pub event_info_1: u8,
    pub event_info_2: u8,
    pub event_info_3: u8,
    pub event_info_4: u8,
    pub event_info_5: u8,
    pub invasion_type: u8,
    pub lobby_id: u64,
    pub sandstorm_severity: f32,
}

impl TerrariaPacket for WorldInfo {
    fn deserialize(bytes: &[u8]) -> WorldInfo {
        named!(parse(&[u8]) -> WorldInfo,
           do_parse!(time: le_i32 >>
                     day_moon_info: le_u8 >>
                     moon_phase: le_u8 >>
                     max_tiles_x: le_i16 >>
                     max_tiles_y: le_i16 >>
                     spawn_x: le_i16 >>
                     spawn_y: le_i16 >>
                     world_surface: le_i16 >>
                     rock_layer: le_i16 >>
                     world_id: le_i32 >>
                     world_name_length: le_u8 >>
                     world_name: take_str!(world_name_length) >>
                     world_unique_id: take!(16) >>
                     moon_type: le_u8 >>
                     tree_background: le_u8 >>
                     corrption_background: le_u8 >>
                     jungle_background: le_u8 >>
                     snow_background: le_u8 >>
                     hallow_background: le_u8 >>
                     crimson_background: le_u8 >>
                     desert_background: le_u8 >>
                     ocean_background: le_u8 >>
                     ice_back_style: le_u8 >>
                     jungle_back_style: le_u8 >>
                     hell_back_style: le_u8 >>
                     wind_speed_set: le_f32 >>
                     cloud_number: le_u8 >>
                     tree_1: le_i32 >>
                     tree_2: le_i32 >>
                     tree_3: le_i32 >>
                     tree_style_1: le_u8 >>
                     tree_style_2: le_u8 >>
                     tree_style_3: le_u8 >>
                     tree_style_4: le_u8 >>
                     cave_back_1: le_i32 >>
                     cave_back_2: le_i32 >>
                     cave_back_3: le_i32 >>
                     cave_back_style_1: le_u8 >>
                     cave_back_style_2: le_u8 >>
                     cave_back_style_3: le_u8 >>
                     cave_back_style_4: le_u8 >>
                     rain: le_f32 >>
                     event_info_1: le_u8 >>
                     event_info_2: le_u8 >>
                     event_info_3: le_u8 >>
                     event_info_4: le_u8 >>
                     event_info_5: le_u8 >>
                     invasion_type: le_u8 >>
                     lobby_id: le_u64 >>
                     sandstorm_severity: le_f32 >>
                     (WorldInfo {
                         time, day_moon_info, moon_phase, max_tiles_x, max_tiles_y, spawn_x,
                         spawn_y, world_surface, rock_layer, world_id, world_name: world_name.to_owned(), world_unique_id: world_unique_id.to_owned(),
                         moon_type, tree_background, corrption_background, jungle_background,
                         snow_background, hallow_background, crimson_background, desert_background,
                         ocean_background, ice_back_style, jungle_back_style, hell_back_style,
                         wind_speed_set, cloud_number, tree_1, tree_2, tree_3, tree_style_1,
                         tree_style_2, tree_style_3, tree_style_4, cave_back_1, cave_back_2,
                         cave_back_3, cave_back_style_1, cave_back_style_2, cave_back_style_3,
                         cave_back_style_4, rain, event_info_1, event_info_2, event_info_3,
                         event_info_4, event_info_5, invasion_type, lobby_id, sandstorm_severity
                     })));
        //trace_macros!(false);
        let x = parse(bytes);
        if x.is_err() {
            println!("error: {}", x.unwrap_err());
        }
        let (_, pkt) = parse(bytes).unwrap();
        pkt
    }
}
