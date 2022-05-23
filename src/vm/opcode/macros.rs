macro_rules! h256_to_u256 {
    ($($v: expr),*) => {
        (
            $(
                U256::from_big_endian(&$v[..])
            ),*
        )
    };
}

macro_rules! u256_to_h256 {
    ($($v: expr),*) => {
        (
            $(
                {
                    let mut val = H256::default();
                    $v.to_big_endian(&mut val[..]);
                    val
                }
            ),*
        )
    };
}

macro_rules! pop {
    ($vm: expr, $($id:ident),+) => {
        $(
            let $id = match $vm.stack.pop() {
                Ok(v) => v,
                Err(e) => return Control::Error(e)
            };
        )*
    };
}

macro_rules! push {
    ($vm: expr, $($v: expr),*) => {
        $(
            match $vm.stack.push($v) {
                Ok(_) => (),
                Err(e) => return Control::Error(e)
            }
        )*
    };
}

macro_rules! peek {
    ($vm: expr, $id:ident, $n:expr) => {
        let $id = match $vm.stack.peek($n) {
            Ok(v) => v,
            Err(e) => return Control::Error(e),
        };
    };
}

macro_rules! pop_u256 {
    ($vm: expr, $($id:ident),*) => {
            $(
                let $id = match $vm.stack.pop() {
                    Ok(v) => h256_to_u256!(v[..]),
                    Err(e) => return Control::Error(e)
                };
            )*
        };
}

macro_rules! push_u256 {
    ($vm: expr, $($v: expr),*) => {
        $(
            let val = u256_to_h256!($v);
            match $vm.stack.push(val) {
                Ok(()) => (),
                Err(e) => return Control::Error(e)
            }
        )*
    };
}

macro_rules! pop_usize {
    ($vm: expr, $($id:ident),*) => {
        $(
            let $id = match $vm.stack.pop() {
                Ok(v) => {
                    let x = h256_to_u256!(v);
                    if x > U256::from(usize::MAX) {
                        return Control::Error(VmError::UnsupportedOperation);
                    }
                    x.as_usize()
                },
                Err(e) => return Control::Error(e)
            };
        )*
    };
}
