use std::convert::TryInto;
use std::io::{Write, Result as IOResult};

pub fn read_u16(buffer: &[u8], cursor: &mut usize) -> Option<u16> {
    let value = u16::from_be_bytes([buffer[*cursor], buffer[*cursor+1]]);
    *cursor = (*cursor)+2;
    Some(value)
}

pub fn read_var_int(buffer: &[u8], cursor: &mut usize) -> Option<i32> {
    // adapted from https://wiki.vg/Protocol#VarInt_and_VarLong
    let mut value = 0i32;
    let mut len = 0isize;
    let mut byte: i32;
    let mut cur = *cursor;
    loop {
        byte = buffer[cur].into();
        cur += 1;
        value |= (byte & 0x7F) << (len * 7);
        len += 1;
        if len > 5 {
            return None;
        }
        if (byte & 0x80) != 0x80 {
            break;
        }
    }
    *cursor = cur;
    Some(value)
}

pub fn write_var_int<T: Write>(mut value: i32, buffer: &mut T) -> IOResult<usize> {
    let mut c = 0;
    loop {
        if value & !0x7F == 0 {
            let b = value as u8;
            c += buffer.write(&[b])?;
            break;
        }
        let b = ((value & 0x7F) | 0x80) as u8;
        c += buffer.write(&[b])?;
        value = value >> 7
    }
    Ok(c)
}

pub fn read_var_long(buffer: &[u8], cursor: &mut usize) -> Option<i64> {
    // adapted from https://wiki.vg/Protocol#VarInt_and_VarLong
    let mut value = 0i64;
    let mut len = 0isize;
    let mut byte: i64;
    let mut cur = *cursor;
    loop {
        byte = buffer[cur].into();
        cur += 1;
        value |= (byte & 0x7F) << (len * 7);
        len += 1;
        if len > 10 {
            return None;
        }
        if (byte & 0x80) != 0x80 {
            break;
        }
    }
    *cursor = cur;
    Some(value)
}

pub fn write_var_long<T: Write>(mut value: i64, buffer: &mut T) -> IOResult<usize> {
    let mut c = 0;
    loop {
        if value & !0x7F == 0 {
            let b = value as u8;
            c += buffer.write(&[b])?;
            break;
        }
        let b = ((value & 0x7F) | 0x80) as u8;
        c += buffer.write(&[b])?;
        value = value >> 7
    }
    Ok(c)
}

pub fn read_string(buffer: &[u8], cursor: &mut usize) -> Option<String> {
    let mut cur = *cursor;
    let len: usize = read_var_int(buffer, &mut cur)?.try_into().ok()?;
    let s = std::str::from_utf8(&buffer[cur..cur+len]).ok()?;
    *cursor = cur+len;
    Some(String::from(s))
}

pub fn write_string<T: Write>(value: &str, buffer: &mut T) -> IOResult<usize> {
    let mut c = write_var_int(value.len().try_into().unwrap(), buffer)?;
    c += buffer.write(value.as_bytes())?;
    Ok(c)
}

#[derive(Debug)]
pub struct Handshake {
    pub proto_version: i32,
    pub server_addr: String,
    pub server_port: u16,
    pub next_state: i32,
}

pub fn read_handshake(buffer: &[u8], cursor: &mut usize) -> Option<Handshake> {
    let mut cur = *cursor;
    let proto_version = read_var_int(buffer, &mut cur)?;
    let server_addr = read_string(buffer, &mut cur)?;
    let server_port = read_u16(buffer, &mut cur)?;
    let next_state = read_var_int(buffer, &mut cur)?;
    *cursor = cur;
    Some(Handshake {
        proto_version,
        server_addr,
        server_port,
        next_state,
    })
}

#[derive(Debug)]
pub struct Packet {
    pub len: i32,
    pub id: i32,
}

pub fn read_packet(buffer: &[u8], cursor: &mut usize) -> Option<Packet> {
    let mut cur = *cursor;
    let len = read_var_int(buffer, &mut cur)?;
    let id = read_var_int(buffer, &mut cur)?;
    *cursor = cur;
    Some(Packet {
        len,
        id,
    })
}

pub fn write_packet<T: Write>(payload: &[u8], id: i32, buffer: &mut T) -> IOResult<usize> {
    let len: i32 = payload.len().try_into().unwrap();
    // note: `len + 1` will fail if packet id is larger than 1 byte.
    let mut c = write_var_int(len + 1, buffer)?;
    c += write_var_int(id, buffer)?;
    c += buffer.write(payload)?;
    Ok(c)
}
