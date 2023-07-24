struct Door {
    is_open: bool
}

impl Door {
    fn new(is_open: bool) -> Door {
	Door { is_open: is_open }
    }
}

trait Openable {
    fn open(&mut self);
}

impl Openable for Door {
    fn open(&mut self) {
	self.is_open = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	let door1 = Door::new(true);
	let door2 = Door::new(true);
	
        assert_eq!(door1.is_open, door2.is_open);
    }

    #[test]
    fn open_door() {
	let mut door = Door::new(false);
	door.open();
	assert!(door.is_open);
    }
}
