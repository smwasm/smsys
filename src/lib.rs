mod sm_io;

pub fn init() -> bool {
    sm_io::_sm_init();
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = init();
        assert_eq!(result, true);
    }
}
