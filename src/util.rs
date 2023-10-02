use screeps::RoomObject;
use wasm_bindgen::JsValue;

pub fn cast_room_object_into<T>(room_object: RoomObject) -> T
where
    T: From<JsValue>,
{
    JsValue::from(room_object).into()
}
