use ratatui::layout::Rect;
#[cfg(test)]
mod subtract {

    #[cfg(test)]
    mod disjoint_rectangles {
        use crate::geometry::rectangle::Rectangle;

        #[test]
        fn returns_only_self() {
            let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
            let rect2 = Rectangle::from_coordinates((15, 15), (20, 20));

            let result = rect1.subtract(&rect2);
            assert_eq!(result, vec![rect1]);
        }
    }

    #[cfg(test)]
    mod identical_rectangles {
        use crate::geometry::rectangle::Rectangle;

        #[test]
        fn returns_empty_vec() {
            let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
            let rect2 = Rectangle::from_coordinates((5, 5), (10, 10));

            let result = rect1.subtract(&rect2);
            assert_eq!(result, vec![]);
        }
    }

    #[cfg(test)]
    mod intersecting_rectangles {
        mod same_width_self_higher_than_other {
            use crate::geometry::rectangle::Rectangle;

            #[test]
            fn returns_the_top_part_of_self() {
                let rect1 = Rectangle::from_coordinates((5, 5), (10, 10));
                let rect2 = Rectangle::from_coordinates((5, 8), (10, 13));

                let result = rect1.subtract(&rect2);
                assert_eq!(result, vec![Rectangle::from_coordinates((5, 5), (10, 8))]);
            }
        }
    }
}
