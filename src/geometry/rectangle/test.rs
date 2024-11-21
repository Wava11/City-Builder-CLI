use ratatui::layout::Rect;
#[cfg(test)]
mod subtract {

    #[cfg(test)]
    mod disjoint_rectangles {
        use crate::geometry::{rectangle::Rectangle, Point};

        
        #[test]
        fn returns_only_self() {
            let rect1 = Rectangle::from_coordinates((5,5),(10,10));
            let rect2 = Rectangle::from_coordinates((15,15),(20,20));
            
            let result = rect1.subtract(&rect2);
            assert_eq!(result, vec![rect1]);
        }
    }
}
