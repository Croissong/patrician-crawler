pub const MATERIALS: [&'static str; 20] = ["grain", "meat", "fish", "beer", "salt", "honey", "spices", "wine", "cloth", "skins", "whale oil", "timber", "iron goods", "leather", "wool", "pitch", "pig iron", "hemp", "pottery", "bricks"];

pub const TOWN_NAMES: [&'static str; 3] = ["Luebeck", "Rostock", "Hamburg"];

pub const CHANNEL: &'static str = "{\"topic\":\"rust_client:web_client\",\"ref\":\"1\",\"payload\":{},\"event\":\"phx_join\"}";

pub const SERVER_URL: &'static str = "ws://localhost:4000/socket/websocket?vsn=1.0.0";

// pub const MSG_FORMAT: &'static str = "{{\"topic\":\"rust_client:web_client\",\"ref\":null,\"payload\":{{\"body\":{}}},\"event\":\"rust_client:web_client\"}}";
