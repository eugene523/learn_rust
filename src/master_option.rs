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
            None => {},
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
            None => {},
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

    #[test]
    pub fn option_match_as_ref_mut() {
        let mut p_opt = Some(Point::new(1.0, 2.0));
        match p_opt.as_mut() {
            Some(p) => { p.x += 1.0; p.y += 1.0; },
            None => {},
        }
        let Point { x, y } = p_opt.unwrap();
        assert!(x == 2.0 && y == 3.0);
    }

    #[test]
    pub fn option_if_let_as_ref_mut() {
        let mut p_opt = Some(Point::new(1.0, 2.0));
        if let Some(p) = p_opt.as_mut() {
            p.x += 1.0;
            p.y += 1.0;
        }
        let Point { x, y } = p_opt.unwrap();
        assert!(x == 2.0 && y == 3.0);
    }

    #[test]
    #[should_panic(expected = "no value")]
    pub fn option_expected() {
        let x = Some(3);
        let y = x.expect("no value");
        assert!(y == 3);

        let x: Option<i32> = None;
        let _y = x.expect("no value");
    }

    #[test]
    #[should_panic]
    pub fn option_unwrap() {
        let x = Some("Hello World!".to_string());
        let y = x.unwrap();
        assert!(y == "Hello World!");

        let x: Option<String> = None;
        let _y = x.unwrap(); // Error here
    }

    #[test]
    pub fn option_unwrap_or() {
        let x = Some(3);
        let y = x.unwrap_or(1);
        assert!(y == 3);

        let x: Option<i32> = None;
        let y = x.unwrap_or(1);
        assert!(y == 1);
    }

    #[test]
    pub fn option_unwrap_or_else() {
        let x = 3;
        
        let x1 = Some(5);
        let y1 = x1.unwrap_or_else(|| 2 * x);
        assert!(y1 == 5);

        let x2: Option<i32> = None;
        let y2 = x2.unwrap_or_else(|| x * x);
        assert!(y2 == 9);
    }

    #[test]
    pub fn option_unwrap_or_default() {
        let x: Option<i32> = Some(5);
        let y: Option<i32> = None;
        assert!(x.unwrap_or_default() == 5);
        assert!(y.unwrap_or_default() == 0);
    }

    #[test]
    pub fn option_unwrap_unchecked() {
        let x = Some("hello");
        assert!(unsafe { x.unwrap_unchecked() == "hello" });
    }

    #[test]
    pub fn option_map() {
        let x: Option<i32> = None;
        let y = x.map(|v| v * v);
        assert!(y.is_none());

        let x = Some(5);
        let y = x.map(|v| v * v);
        assert!(y.unwrap() == 25);
    }

    #[test]
    pub fn option_map_or() {
        let x = Some("hello");
        let l = x.map_or(0, |v| v.len());
        assert!(l == 5);

        let x: Option<&str> = None;
        let l = x.map_or(10, |v| v.len());
        assert!(l == 10);
    }

    #[test]
    pub fn option_map_or_else() {
        let x = 3;
        let s = Some("hello");
        let l = s.map_or_else(|| 2 * x, |v| v.len());
        assert!(l == 5);

        let s: Option<&str> = None;
        let l = s.map_or_else(|| 2 * x, |v| v.len());
        assert!(l == 6);
    }
}