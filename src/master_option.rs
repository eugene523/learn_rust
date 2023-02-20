pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn create_and_match() {
        let p_opt = Some(Point::new(1.0, 2.0));

        // match забирает p_opt, так что далее мы его использовать не можем.
        match p_opt {
            Some(p) => { assert!(p.x == 1.0 && p.y == 2.0); }
            None => (),
        }

        // Нельзя:
        // let Point { x, y } = p_opt.unwrap();
    }

    #[test]
    pub fn create_and_if_let() {
        let p_opt = Some(Point::new(1.0, 2.0));
        
        // if let забирает p_opt, так что далее мы его использовать не можем.
        if let Some(p) = p_opt {
            assert!(p.x == 1.0 && p.y == 2.0);
        }

        // Нельзя:
        // let Point { x, y } = p_opt.unwrap();
    }

    #[test]
    pub fn create_and_match_ref() {
        let p_opt = Some(Point::new(1.0, 2.0));

        // Деструкция по ссылке ref p. Далее можно использовать.
        match p_opt {
            Some(ref p) => { assert!(p.x == 1.0 && p.y == 2.0); },
            None => ()
        }

        let Point { x, y } = p_opt.unwrap();
        assert!(x == 1.0 && y == 2.0);
    }

    #[test]
    pub fn create_and_if_let_ref() {
        let p_opt = Some(Point::new(1.0, 2.0));

        if let Some(ref p) = p_opt {
            assert!(p.x == 1.0 && p.y == 2.0);
        }

        let Point { x, y } = p_opt.unwrap();
        assert!(x == 1.0 && y == 2.0);
    }

    #[test]
    pub fn create_and_match_ref_mutf() {
        let mut p_opt = Some(Point::new(1.0, 2.0));

        match p_opt {
            Some(ref mut p) => { p.x += 1.0; p.y += 1.0; },
            None => ()
        }

        let Point { x, y } = p_opt.unwrap();
        assert!(x == 2.0 && y == 3.0);
    }

    #[test]
    pub fn create_and_match_if_let_ref_mut() {
        let mut p_opt = Some(Point::new(1.0, 2.0));
        
        if let Some(ref mut p) = p_opt {
            p.x += 1.0;
            p.y += 1.0;
        }

        let Point { x, y } = p_opt.unwrap();
        assert!(x == 2.0 && y == 3.0);
    }

    #[test]
    pub fn option_is() {
        let x = Some(3);
        assert_eq!(x.is_some(), true);
        assert_eq!(x.is_none(), false);

        let x: Option<i32> = None;
        assert_eq!(x.is_some(), false);
        assert_eq!(x.is_none(), true);
    }

    #[test]
    pub fn option_match_as_ref() {
        let p_opt = Some(Point::new(1.0, 2.0));
        match p_opt.as_ref() {
            Some(p) => { assert!(p.x == 1.0 && p.y == 2.0); },
            None => {},
        }
        let Point { x, y } = p_opt.unwrap();
        assert!(x == 1.0 && y == 2.0);
    }

    #[test]
    pub fn option_if_let_as_ref() {
        let p_opt = Some(Point::new(1.0, 2.0));
        if let Some(p) = p_opt.as_ref() {
            assert!(p.x == 1.0 && p.y == 2.0);
        }
        let Point { x, y } = p_opt.unwrap();
        assert!(x == 1.0 && y == 2.0);
    }

}