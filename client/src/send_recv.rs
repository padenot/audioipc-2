use cubeb_core::Error;
use cubeb_core::ffi;

#[doc(hidden)]
pub fn _err<E>(e: E) -> Error
where
    E: Into<Option<ffi::cubeb_error_code>>
{
    match e.into() {
        Some(e) => unsafe { Error::from_raw(e) },
        None => Error::new()
    }
}

#[macro_export]
macro_rules! send_recv {
    ($rpc:expr, $smsg:ident => $rmsg:ident) => {{
        let resp = send_recv!(__send $rpc, $smsg);
        send_recv!(__recv resp, $rmsg)
    }};
    ($rpc:expr, $smsg:ident => $rmsg:ident()) => {{
        let resp = send_recv!(__send $rpc, $smsg);
        send_recv!(__recv resp, $rmsg __result)
    }};
    ($rpc:expr, $smsg:ident($($a:expr),*) => $rmsg:ident) => {{
        let resp = send_recv!(__send $rpc, $smsg, $($a),*);
        send_recv!(__recv resp, $rmsg)
    }};
    ($rpc:expr, $smsg:ident($($a:expr),*) => $rmsg:ident()) => {{
        let resp = send_recv!(__send $rpc, $smsg, $($a),*);
        send_recv!(__recv resp, $rmsg __result)
    }};
    //
    (__send $rpc:expr, $smsg:ident) => ({
        $rpc.call(ServerMessage::$smsg)
    });
    (__send $rpc:expr, $smsg:ident, $($a:expr),*) => ({
        $rpc.call(ServerMessage::$smsg($($a),*))
    });
    (__recv $resp:expr, $rmsg:ident) => ({
        match $resp.wait() {
            Ok(ClientMessage::$rmsg) => Ok(()),
            Ok(ClientMessage::Error(e)) => Err($crate::send_recv::_err(e)),
            Ok(m) => {
                debug!("received wrong message - got={:?}", m);
                Err($crate::send_recv::_err(None))
            },
            Err(e) => {
                debug!("received error from rpc - got={:?}", e);
                Err($crate::send_recv::_err(None))
            },
        }
    });
    (__recv $resp:expr, $rmsg:ident __result) => ({
        match $resp.wait() {
            Ok(ClientMessage::$rmsg(v)) => Ok(v),
            Ok(ClientMessage::Error(e)) => Err($crate::send_recv::_err(e)),
            Ok(m) => {
                debug!("received wrong message - got={:?}", m);
                Err($crate::send_recv::_err(None))
            },
            Err(e) => {
                debug!("received error - got={:?}", e);
                Err($crate::send_recv::_err(None))
            },
        }
    })
}
