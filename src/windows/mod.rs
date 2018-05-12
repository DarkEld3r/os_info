mod winapi;

use Info;

pub fn current_platform() -> Info {
    trace!("windows::current_platform is called");
    let info = winapi::get();
    trace!("Returning {:?}", info);
    info
}

#[cfg(test)]
mod tests {
    use super::*;
    use Type;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::Windows, version.os_type());
    }
}
