use anyhow::{anyhow, Result};

pub fn assert_eq<T: PartialEq + std::fmt::Debug>(l: T, r: T) -> Result<()> {
    if l.eq(&r) {
        Ok(())
    } else {
        Err(anyhow!(
            "A left param doesn't equal a right param\n\tA left param: {:#?}\n\tA right param: {:#?}",
            l,
            r
        ))
    }
}
